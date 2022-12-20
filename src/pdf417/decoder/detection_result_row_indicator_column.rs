/*
 * Copyright 2013 ZXing authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::fmt::Display;

use crate::{pdf417::pdf_417_common, ResultPoint};

use super::{BarcodeMetadata, BarcodeValue, BoundingBox, Codeword, DetectionRXingResultColumn};

/**
 * @author Guenther Grau
 */
pub struct DetectionRXingResultRowIndicatorColumn<'a>(DetectionRXingResultColumn<'a>, bool);
impl<'a> DetectionRXingResultRowIndicatorColumn<'_> {
    // private final boolean isLeft;

    pub fn new(
        boundingBox: &'a BoundingBox,
        isLeft: bool,
    ) -> DetectionRXingResultRowIndicatorColumn<'a> {
        DetectionRXingResultRowIndicatorColumn(DetectionRXingResultColumn::new(boundingBox), isLeft)
    }

    fn setRowNumbers(&mut self) {
        for codeword_opt in self.0.getCodewordsMut() {
            // for (Codeword codeword : getCodewords()) {
            if let Some(codeword) = codeword_opt {
                // if (codeword != null) {
                codeword.setRowNumberAsRowIndicatorColumn();
            }
        }
    }

    // TODO implement properly
    // TODO maybe we should add missing codewords to store the correct row number to make
    // finding row numbers for other columns easier
    // use row height count to make detection of invalid row numbers more reliable
    pub fn adjustCompleteIndicatorColumnRowNumbers(&mut self, barcodeMetadata: &BarcodeMetadata) {
        // let codewords = self.0.getCodewordsMut();
        self.setRowNumbers();
        Self::removeIncorrectCodewords(self.0.getCodewordsMut(), barcodeMetadata, self.1);
        let boundingBox = self.0.getBoundingBox();
        let top = if self.1 {
            boundingBox.getTopLeft()
        } else {
            boundingBox.getTopRight()
        };
        let bottom = if self.1 {
            boundingBox.getBottomLeft()
        } else {
            boundingBox.getBottomRight()
        };
        let firstRow = self.0.imageRowToCodewordIndex(top.getY() as u32);
        let lastRow = self.0.imageRowToCodewordIndex(bottom.getY() as u32);
        // We need to be careful using the average row height. Barcode could be skewed so that we have smaller and
        // taller rows
        //float averageRowHeight = (lastRow - firstRow) / (float) barcodeMetadata.getRowCount();
        let mut barcodeRow = -1;
        let mut maxRowHeight = 1;
        let mut currentRowHeight = 0;
        for codewordsRow in firstRow..lastRow {
            // for (int codewordsRow = firstRow; codewordsRow < lastRow; codewordsRow++) {
            if let Some(codeword) = self.0.getCodewordsMut()[codewordsRow] {
                // if (codewords[codewordsRow] == null) {
                //   continue;
                // }
                // let codeword = codewords[codewordsRow];

                let rowDifference = codeword.getRowNumber() - barcodeRow;

                // TODO improve handling with case where first row indicator doesn't start with 0

                if rowDifference == 0 {
                    currentRowHeight += 1;
                } else if rowDifference == 1 {
                    maxRowHeight = maxRowHeight.max(currentRowHeight);
                    currentRowHeight = 1;
                    barcodeRow = codeword.getRowNumber();
                } else if rowDifference < 0
                    || codeword.getRowNumber() >= barcodeMetadata.getRowCount() as i32
                    || rowDifference > codewordsRow as i32
                {
                    self.0.getCodewordsMut()[codewordsRow] = None;
                } else {
                    let checkedRows;
                    if maxRowHeight > 2 {
                        checkedRows = (maxRowHeight - 2) * rowDifference;
                    } else {
                        checkedRows = rowDifference;
                    }
                    let mut closePreviousCodewordFound = checkedRows >= codewordsRow as i32;
                    let mut i = 1;
                    while i <= checkedRows && !closePreviousCodewordFound {
                        // for (int i = 1; i <= checkedRows && !closePreviousCodewordFound; i++) {
                        // there must be (height * rowDifference) number of codewords missing. For now we assume height = 1.
                        // This should hopefully get rid of most problems already.
                        closePreviousCodewordFound =
                            self.0.getCodewords()[codewordsRow as usize - i as usize].is_some();

                        i += 1;
                    }
                    if closePreviousCodewordFound {
                        self.0.getCodewordsMut()[codewordsRow] = None;
                    } else {
                        barcodeRow = codeword.getRowNumber();
                        currentRowHeight = 1;
                    }
                }
            } else {
                continue;
            }
        }
        //return (int) (averageRowHeight + 0.5);
    }

    pub fn getRowHeights(&mut self) -> Option<Vec<u32>> {
        if let Some(barcodeMetadata) = self.getBarcodeMetadata() {
            self.adjustIncompleteIndicatorColumnRowNumbers(&barcodeMetadata);
            let mut result = vec![0; barcodeMetadata.getRowCount() as usize];
            for codeword_opt in self.0.getCodewords() {
                // for (Codeword codeword : getCodewords()) {
                if let Some(codeword) = codeword_opt {
                    let rowNumber = codeword.getRowNumber();
                    if rowNumber as usize >= result.len() {
                        // We have more rows than the barcode metadata allows for, ignore them.
                        continue;
                    }
                    result[rowNumber as usize] += 1;
                }
                // else throw exception?
                else {
                    continue;
                }
            }
            Some(result)
        } else {
            None
        }
    }

    // TODO maybe we should add missing codewords to store the correct row number to make
    // finding row numbers for other columns easier
    // use row height count to make detection of invalid row numbers more reliable
    fn adjustIncompleteIndicatorColumnRowNumbers(&mut self, barcodeMetadata: &BarcodeMetadata) {
        let boundingBox = self.0.getBoundingBox();
        let top = if self.1 {
            boundingBox.getTopLeft()
        } else {
            boundingBox.getTopRight()
        };
        let bottom = if self.1 {
            boundingBox.getBottomLeft()
        } else {
            boundingBox.getBottomRight()
        };
        let firstRow = self.0.imageRowToCodewordIndex(top.getY() as u32);
        let lastRow = self.0.imageRowToCodewordIndex(bottom.getY() as u32);
        //float averageRowHeight = (lastRow - firstRow) / (float) barcodeMetadata.getRowCount();
        let codewords = self.0.getCodewordsMut();
        let mut barcodeRow = -1;
        let mut maxRowHeight = 1;
        let mut currentRowHeight = 0;
        for codewordsRow in firstRow..lastRow {
            // for (int codewordsRow = firstRow; codewordsRow < lastRow; codewordsRow++) {

            if let Some(codeword) = &mut codewords[codewordsRow] {
                codeword.setRowNumberAsRowIndicatorColumn();

                let rowDifference = codeword.getRowNumber() - barcodeRow;

                // TODO improve handling with case where first row indicator doesn't start with 0

                if rowDifference == 0 {
                    currentRowHeight += 1;
                } else if rowDifference == 1 {
                    maxRowHeight = maxRowHeight.max(currentRowHeight);
                    currentRowHeight = 1;
                    barcodeRow = codeword.getRowNumber();
                } else if codeword.getRowNumber() >= barcodeMetadata.getRowCount() as i32 {
                    codewords[codewordsRow] = None;
                } else {
                    barcodeRow = codeword.getRowNumber();
                    currentRowHeight = 1;
                }
            } else {
                continue;
            }
        }
        //return (int) (averageRowHeight + 0.5);
    }

    pub fn getBarcodeMetadata(&mut self) -> Option<BarcodeMetadata> {
        let codewords = self.0.getCodewordsMut();
        let mut barcodeColumnCount = BarcodeValue::new();
        let mut barcodeRowCountUpperPart = BarcodeValue::new();
        let mut barcodeRowCountLowerPart = BarcodeValue::new();
        let mut barcodeECLevel = BarcodeValue::new();
        for codeword_opt in codewords.iter_mut() {
            // for (Codeword codeword : codewords) {
            if let Some(codeword) = codeword_opt {
                codeword.setRowNumberAsRowIndicatorColumn();
                let rowIndicatorValue = codeword.getValue() % 30;
                let mut codewordRowNumber = codeword.getRowNumber();
                if !self.1 {
                    codewordRowNumber += 2;
                }
                match codewordRowNumber % 3 {
                    0 => barcodeRowCountUpperPart.setValue(rowIndicatorValue * 3 + 1),
                    1 => {
                        barcodeECLevel.setValue(rowIndicatorValue / 3);
                        barcodeRowCountLowerPart.setValue(rowIndicatorValue % 3);
                    }
                    2 => barcodeColumnCount.setValue(rowIndicatorValue + 1),
                    _ => {}
                }
            } else {
                continue;
            }
        }
        // Maybe we should check if we have ambiguous values?
        if (barcodeColumnCount.getValue().len() == 0)
            || (barcodeRowCountUpperPart.getValue().len() == 0)
            || (barcodeRowCountLowerPart.getValue().len() == 0)
            || (barcodeECLevel.getValue().len() == 0)
            || barcodeColumnCount.getValue()[0] < 1
            || barcodeRowCountUpperPart.getValue()[0] + barcodeRowCountLowerPart.getValue()[0]
                < pdf_417_common::MIN_ROWS_IN_BARCODE
            || barcodeRowCountUpperPart.getValue()[0] + barcodeRowCountLowerPart.getValue()[0]
                > pdf_417_common::MAX_ROWS_IN_BARCODE
        {
            return None;
        }
        let barcodeMetadata = BarcodeMetadata::new(
            barcodeColumnCount.getValue()[0],
            barcodeRowCountUpperPart.getValue()[0],
            barcodeRowCountLowerPart.getValue()[0],
            barcodeECLevel.getValue()[0],
        );
        Self::removeIncorrectCodewords(codewords, &barcodeMetadata, self.1);

        Some(barcodeMetadata)
    }

    fn removeIncorrectCodewords(
        codewords: &mut [Option<Codeword>],
        barcodeMetadata: &BarcodeMetadata,
        isLeft: bool,
    ) {
        // Remove codewords which do not match the metadata
        // TODO Maybe we should keep the incorrect codewords for the start and end positions?
        for codewordRow in 0..codewords.len() {
            // for (int codewordRow = 0; codewordRow < codewords.length; codewordRow++) {
            if let Some(codeword) = codewords[codewordRow] {
                let rowIndicatorValue = codeword.getValue() % 30;
                let mut codewordRowNumber = codeword.getRowNumber();
                if codewordRowNumber > barcodeMetadata.getRowCount() as i32 {
                    codewords[codewordRow] = None;
                    continue;
                }
                if !isLeft {
                    codewordRowNumber += 2;
                }
                match codewordRowNumber % 3 {
                    0 => {
                        if rowIndicatorValue * 3 + 1 != barcodeMetadata.getRowCountUpperPart() {
                            codewords[codewordRow] = None;
                        }
                    }
                    1 => {
                        if rowIndicatorValue / 3 != barcodeMetadata.getErrorCorrectionLevel()
                            || rowIndicatorValue % 3 != barcodeMetadata.getRowCountLowerPart()
                        {
                            codewords[codewordRow] = None;
                        }
                    }
                    2 => {
                        if rowIndicatorValue + 1 != barcodeMetadata.getColumnCount() {
                            codewords[codewordRow] = None;
                        }
                    }
                    _ => {}
                }
            } else {
                continue;
            }
        }
    }

    pub fn isLeft(&self) -> bool {
        self.1
    }
}

impl Display for DetectionRXingResultRowIndicatorColumn<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IsLeft: {} \n {}", self.1, self.0)
    }
}
