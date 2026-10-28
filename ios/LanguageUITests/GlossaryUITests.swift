import XCTest

/// The glossary, the progress page, settings and the glossary's index rail.
final class GlossaryUITests: LanguageUITestCase {
    func testGlossaryAndProgressAndSettingsOpen() throws {
        XCTAssertTrue(app.buttons["unit-\(Self.practicedUnitId)"].waitForExistence(timeout: 20))

        app.tabBars.buttons["Glossary"].tap()
        XCTAssertTrue(app.staticTexts["Glossary"].waitForExistence(timeout: 5))
        saveScreenshot("09-glossary")

        app.tabBars.buttons["Progress"].tap()
        XCTAssertTrue(app.staticTexts["Review deck"].waitForExistence(timeout: 5))
        saveScreenshot("10-progress")

        app.buttons["settings"].firstMatch.tap()
        XCTAssertTrue(app.buttons["voice-kokoro"].waitForExistence(timeout: 5))

        let publicationLabel: XCUIElement = app.staticTexts["publication-label"]

        XCTAssertTrue(publicationLabel.waitForExistence(timeout: 10))
        XCTAssertTrue(publicationLabel.label.hasPrefix("Published as "), publicationLabel.label)
        saveScreenshot("11-settings")
    }

    @MainActor
    func testTheGlossaryLevelRailJumpsToTheFirstWordOfALevel() throws {
        let firstLevelUnitId: String = try Self.readFirstUnitId(levelCode: Self.jumpedGlossaryLevelCode)
        let firstLevelUnitNumber: Int = try Self.readUnitNumber(unitId: firstLevelUnitId)
        let firstLevelWord: String = try Self.readFirstVocabularyTarget(unitId: firstLevelUnitId)

        openTheGlossary()

        let levelEntry: XCUIElement = glossaryLevelRailEntry(levelCode: Self.jumpedGlossaryLevelCode)
        let firstLevelUnitEntry: XCUIElement = glossaryUnitRailEntry(unitNumber: firstLevelUnitNumber)

        XCTAssertTrue(levelEntry.waitForExistence(timeout: 5))
        XCTAssertEqual(levelEntry.label, "Go to level \(Self.jumpedGlossaryLevelCode)")

        app.buttons[Self.jumpedGlossaryLevelCode].firstMatch.tap()
        XCTAssertTrue(firstLevelUnitEntry.waitForExistence(timeout: 5))
        assertGone(levelEntry, timeout: 5)
        app.buttons["All levels"].firstMatch.tap()
        XCTAssertTrue(levelEntry.waitForExistence(timeout: 5))
        assertGone(firstLevelUnitEntry, timeout: 5)

        withTheIndexRailShown {
            levelEntry.tap()
            self.saveScreenshot("22-glossary-level-index-jump")
        }

        assertNearTopOfThePage(app.staticTexts[firstLevelWord].firstMatch)
        assertSelected(levelEntry)
    }

    @MainActor
    func testTheGlossaryUnitRailOfOneLevelJumpsToTheFirstWordOfAUnit() throws {
        let unitNumber: Int = try Self.readUnitNumber(unitId: Self.jumpedGlossaryUnitId)
        let firstUnitWord: String = try Self.readFirstVocabularyTarget(unitId: Self.jumpedGlossaryUnitId)

        openTheGlossary()
        app.buttons[Self.jumpedGlossaryLevelCode].firstMatch.tap()

        let unitEntry: XCUIElement = glossaryUnitRailEntry(unitNumber: unitNumber)

        XCTAssertTrue(unitEntry.waitForExistence(timeout: 5))
        XCTAssertEqual(unitEntry.label, "Go to unit \(Self.jumpedGlossaryLevelCode) \(unitNumber)")
        XCTAssertFalse(app.staticTexts[firstUnitWord].firstMatch.isHittable)

        withTheIndexRailShown {
            unitEntry.tap()
            self.saveScreenshot("23-glossary-unit-index-jump")
        }

        assertNearTopOfThePage(app.staticTexts[firstUnitWord].firstMatch)
        assertSelected(unitEntry)
    }
}
