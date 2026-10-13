package ro.zach.language.review

import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Test
import ro.zach.language.core.CourseOutlineRecord
import ro.zach.language.core.PhrasePairRecord
import ro.zach.language.core.ReviewDirectionRecord
import ro.zach.language.core.UnitOutlineRecord
import ro.zach.language.core.VocabularyItemRecord
import kotlin.random.Random

private const val MIXED_CARD_COUNT: Int = 64
private const val MIXED_SEED: Int = 7

class ReviewItemsTest {
    private val cardIds: List<String> = listOf("card-1", "card-2", "card-3")

    private fun vocabularyItem(cardId: String): VocabularyItemRecord {
        return VocabularyItemRecord(
            cardId = cardId,
            target = "alvo",
            source = "target",
            sourceLiteral = null,
            partOfSpeech = null,
            note = null,
            clipKey = null,
            example = PhrasePairRecord(target = "exemplo", source = "example", sourceLiteral = null, clipKey = null),
        )
    }

    private fun unit(unitId: String, cardIds: List<String>): UnitOutlineRecord {
        return UnitOutlineRecord(
            id = unitId,
            levelCode = "A1",
            order = 1u,
            title = unitId,
            objectives = emptyList(),
            vocabulary = cardIds.map { cardId -> vocabularyItem(cardId) },
        )
    }

    @Test
    fun buildReviewItems_keepsTheCardOrderAndFacesTheDirectionsFirstSide() {
        val targetFirst: List<ReviewItem> = buildReviewItems(cardIds, ReviewDirectionRecord.TARGET_TO_SOURCE)
        val sourceFirst: List<ReviewItem> = buildReviewItems(cardIds, ReviewDirectionRecord.SOURCE_TO_TARGET)

        assertEquals(cardIds, targetFirst.map { item -> item.cardId })
        assertTrue(targetFirst.all { item -> item.promptSide == ReviewCardSide.Target })
        assertEquals(cardIds, sourceFirst.map { item -> item.cardId })
        assertTrue(sourceFirst.all { item -> item.promptSide == ReviewCardSide.Source })
    }

    @Test
    fun buildReviewItems_picksASidePerCardWhenMixed() {
        val manyCardIds: List<String> = (1..MIXED_CARD_COUNT).map { number -> "card-$number" }
        val mixed: List<ReviewItem> = buildReviewItems(manyCardIds, ReviewDirectionRecord.MIXED, Random(MIXED_SEED))
        val promptSides: Set<ReviewCardSide> = mixed.map { item -> item.promptSide }.toSet()

        assertEquals(setOf(ReviewCardSide.Target, ReviewCardSide.Source), promptSides)
    }

    @Test
    fun vocabularyIndex_findsEachCardWithItsUnit() {
        val firstUnit: UnitOutlineRecord = unit("a1-01", listOf("card-1"))
        val secondUnit: UnitOutlineRecord = unit("a1-02", listOf("card-2", "card-3"))
        val outline: CourseOutlineRecord = CourseOutlineRecord(
            id = "course",
            title = "Course",
            targetLanguage = "pt-BR",
            sourceLanguage = "en",
            targetLanguageName = "Portuguese",
            sourceLanguageName = "English",
            levels = emptyList(),
            units = listOf(firstUnit, secondUnit),
        )
        val vocabularyIndex: VocabularyIndex = VocabularyIndex(outline)

        assertEquals(firstUnit, vocabularyIndex.entry("card-1")?.unit)
        assertEquals(secondUnit, vocabularyIndex.entry("card-3")?.unit)
        assertEquals("card-3", vocabularyIndex.entry("card-3")?.item?.cardId)
        assertNull(vocabularyIndex.entry("missing"))
    }
}
