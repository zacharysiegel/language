package ro.zach.language.review

import ro.zach.language.core.CourseOutlineRecord
import ro.zach.language.core.ReviewDirectionRecord
import ro.zach.language.core.UnitOutlineRecord
import ro.zach.language.core.VocabularyItemRecord
import java.io.Serializable
import kotlin.random.Random

const val REVIEW_SESSION_CARD_LIMIT: Int = 20

/** The side of a card that faces the learner first. */
enum class ReviewCardSide {
    Target,
    Source,
}

/** A card to show in a review session. */
data class ReviewItem(val cardId: String, val promptSide: ReviewCardSide) : Serializable

/** A vocabulary item of the course and the unit it belongs to. */
data class VocabularyIndexEntry(val unit: UnitOutlineRecord, val item: VocabularyItemRecord)

/** Every vocabulary item of the course by its review card id. */
class VocabularyIndex(outline: CourseOutlineRecord) {
    private val entriesByCardId: Map<String, VocabularyIndexEntry> = buildMap {
        for (unit in outline.units) {
            for (item in unit.vocabulary) {
                put(item.cardId, VocabularyIndexEntry(unit = unit, item = item))
            }
        }
    }

    fun entry(cardId: String): VocabularyIndexEntry? {
        return entriesByCardId[cardId]
    }
}

/** The cards in order, each facing the side `direction` names; a mixed direction picks a side per card. */
fun buildReviewItems(cardIds: List<String>, direction: ReviewDirectionRecord, random: Random = Random.Default): List<ReviewItem> {
    return cardIds.map { cardId: String ->
        val promptSide: ReviewCardSide = when (direction) {
            ReviewDirectionRecord.TARGET_TO_SOURCE -> ReviewCardSide.Target
            ReviewDirectionRecord.SOURCE_TO_TARGET -> ReviewCardSide.Source
            ReviewDirectionRecord.MIXED -> if (random.nextBoolean()) ReviewCardSide.Target else ReviewCardSide.Source
        }

        ReviewItem(cardId = cardId, promptSide = promptSide)
    }
}
