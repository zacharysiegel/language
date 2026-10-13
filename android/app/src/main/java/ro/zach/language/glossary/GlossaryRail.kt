package ro.zach.language.glossary

import ro.zach.language.design.IndexRailEntry

/** Which divisions of the glossary its index rail lists. */
enum class GlossaryRailKind {
    Levels,
    Units;

    /** Entries with the same key belong to the same rail entry. */
    fun railKey(entry: GlossaryEntry): String {
        return when (this) {
            Levels -> entry.unit.levelCode
            Units -> entry.unit.id
        }
    }

    fun railEntry(entry: GlossaryEntry): IndexRailEntry {
        return when (this) {
            Levels -> IndexRailEntry(visibleText = entry.unit.levelCode, accessibilityLabel = "Go to level ${entry.unit.levelCode}")
            Units -> IndexRailEntry(visibleText = entry.unitNumberText(), accessibilityLabel = "Go to unit ${entry.unitLabel()}")
        }
    }

    fun railAccessibilityLabel(): String {
        return when (this) {
            Levels -> "Level index"
            Units -> "Unit index"
        }
    }

    fun railTestTag(): String {
        return when (this) {
            Levels -> "glossary-level-index"
            Units -> "glossary-unit-index"
        }
    }

    companion object {
        /** `levelFilter` is the shown level's code, `null` for all levels. */
        fun forLevelFilter(levelFilter: String?): GlossaryRailKind {
            return if (levelFilter == null) Levels else Units
        }
    }
}

/** Consecutive shown entries of one rail entry. */
data class GlossaryRun(
    /** Where the run's first entry is among the shown entries. */
    val firstEntryIndex: Int,
    val entries: List<GlossaryEntry>,
    /** Set on the first run of each rail entry. */
    val indexRailEntryNumber: Int?,
)

/** The glossary's shown entries in runs, with the index rail entries that jump to them. */
data class GlossaryRail(
    val runs: List<GlossaryRun>,
    /** In order of their first shown entry. */
    val entries: List<IndexRailEntry>,
) {
    /** The shown entry each rail entry jumps to, by rail entry number. */
    fun firstEntryIndexByEntryNumber(): Map<Int, Int> {
        val firstEntryIndexes: MutableMap<Int, Int> = LinkedHashMap()

        for (run in runs) {
            val entryNumber: Int = run.indexRailEntryNumber
                ?: continue

            firstEntryIndexes[entryNumber] = run.firstEntryIndex
        }

        return firstEntryIndexes
    }

    companion object {
        fun dividing(shownEntries: List<GlossaryEntry>, kind: GlossaryRailKind): GlossaryRail {
            val runs: MutableList<GlossaryRun> = ArrayList()
            val railEntries: MutableList<IndexRailEntry> = ArrayList()
            val listedRailKeys: MutableSet<String> = HashSet()
            var runStart: Int = 0

            while (runStart < shownEntries.size) {
                val firstEntry: GlossaryEntry = shownEntries[runStart]
                val railKey: String = kind.railKey(firstEntry)
                val runEnd: Int = runEndIndex(shownEntries, runStart, railKey, kind)
                val startsItsRailEntry: Boolean = listedRailKeys.add(railKey)

                if (startsItsRailEntry) {
                    railEntries.add(kind.railEntry(firstEntry))
                }

                runs.add(
                    GlossaryRun(
                        firstEntryIndex = runStart,
                        entries = shownEntries.subList(runStart, runEnd),
                        indexRailEntryNumber = if (startsItsRailEntry) railEntries.size else null,
                    ),
                )

                runStart = runEnd
            }

            return GlossaryRail(runs = runs, entries = railEntries)
        }

        private fun runEndIndex(shownEntries: List<GlossaryEntry>, runStart: Int, railKey: String, kind: GlossaryRailKind): Int {
            var runEnd: Int = runStart

            while (runEnd < shownEntries.size && kind.railKey(shownEntries[runEnd]) == railKey) {
                runEnd += 1
            }

            return runEnd
        }
    }
}
