import XCTest

/// The index rail beside a unit tab's boxes.
final class IndexRailUITests: LanguageUITestCase {
    @MainActor
    func testPrinciplesNotesAreLabelledAndTheIndexRailJumpsToThem() throws {
        let unit: [String: Any] = try Self.readUnit(Self.firstUnitId)
        let noteCount: Int = (unit["grammar"] as? [[String: Any]] ?? []).count

        openUnit(Self.firstUnitId)
        app.buttons["tab-grammar"].tap()

        let itemLabels: XCUIElementQuery = app.staticTexts.matching(identifier: "unit-item-label")

        XCTAssertTrue(itemLabels.firstMatch.waitForExistence(timeout: 5))
        XCTAssertEqual(itemLabels.count, noteCount)
        XCTAssertEqual(itemLabels.element(boundBy: 0).label, "\(Self.firstUnitLabelPrefix)1")
        XCTAssertEqual(itemLabels.element(boundBy: 1).label, "\(Self.firstUnitLabelPrefix)2")
        assertSelected(railEntry(1))

        let jumpedLabel: XCUIElement = itemLabels.element(boundBy: Self.jumpedNoteNumber - 1)

        XCTAssertFalse(jumpedLabel.isHittable)
        XCTAssertEqual(railEntry(Self.jumpedNoteNumber).label, "Go to note \(Self.firstUnitLabelPrefix)\(Self.jumpedNoteNumber)")

        withTheIndexRailShown {
            self.railEntry(Self.jumpedNoteNumber).tap()
        }

        assertNearTopOfThePage(jumpedLabel)
        assertSelected(railEntry(Self.jumpedNoteNumber))
        saveScreenshot("17-unit-item-index-jump")

        withTheIndexRailShown {
            self.railEntry(1).press(forDuration: 0.1, thenDragTo: self.railEntry(noteCount))
        }

        XCTAssertTrue(itemLabels.element(boundBy: noteCount - 1).isHittable)
        assertSelected(railEntry(noteCount))
        saveScreenshot("18-unit-item-index-drag")
    }

    @MainActor
    func testTheIndexRailIsShownOnlyWhileScrollingAndTakesNoWidth() throws {
        openUnit(Self.firstUnitId)
        app.buttons["tab-grammar"].tap()
        XCTAssertTrue(app.staticTexts["unit-item-label"].firstMatch.waitForExistence(timeout: 5))
        assertTheIndexRailLetsTouchesThrough(itemNumber: Self.jumpedNoteNumber)
        saveScreenshot("16-unit-item-index-at-rest")

        assertTheContentIsInsetEquallyFromBothEdges()

        performWithoutWaitingForTheAppToIdle {
            self.app.swipeUp(velocity: .slow)
            XCTAssertTrue(self.railEntry(1).isHittable)
            self.assertTheIndexRailShadowClearsTheTrailingEdge()
            self.saveScreenshot("16-unit-item-index-while-scrolling")
        }

        Thread.sleep(forTimeInterval: Self.indexRailHideAllowance)
        assertTheIndexRailLetsTouchesThrough(itemNumber: Self.jumpedNoteNumber)
    }

    @MainActor
    func testTheIndexRailJumpsDuringMomentumScrolling() throws {
        openUnit(Self.firstUnitId)
        app.buttons["tab-grammar"].tap()

        let itemLabels: XCUIElementQuery = app.staticTexts.matching(identifier: "unit-item-label")
        let jumpedLabel: XCUIElement = itemLabels.element(boundBy: Self.momentumJumpedNoteNumber - 1)

        XCTAssertTrue(itemLabels.firstMatch.waitForExistence(timeout: 5))

        performWithoutWaitingForTheAppToIdle {
            self.app.swipeUp(velocity: .fast)
            self.railEntry(Self.momentumJumpedNoteNumber).tap()
        }

        assertSelected(railEntry(Self.momentumJumpedNoteNumber))
        assertNearTopOfThePage(jumpedLabel)
        Thread.sleep(forTimeInterval: Self.momentumSettleDuration)
        assertNearTopOfThePage(jumpedLabel)
        saveScreenshot("21-unit-item-index-momentum-jump")
    }
}
