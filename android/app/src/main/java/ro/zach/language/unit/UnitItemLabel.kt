package ro.zach.language.unit

import ro.zach.language.core.UnitOutlineRecord
import ro.zach.language.core.unitItemLabel

/** The label of the unit's `itemNumber`th note, dialogue or reading, such as "A1.1.4". */
fun UnitOutlineRecord.itemLabel(itemNumber: Int): String {
    return unitItemLabel(levelCode, order, itemNumber.toUInt())
}
