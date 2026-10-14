package ro.zach.language.design

import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.interaction.collectIsPressedAsState
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.RowScope
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.widthIn
import androidx.compose.foundation.layout.wrapContentWidth
import androidx.compose.foundation.selection.toggleable
import androidx.compose.foundation.text.BasicText
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.alpha
import androidx.compose.ui.draw.drawBehind
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.ColorFilter
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.semantics.Role
import androidx.compose.ui.semantics.heading
import androidx.compose.ui.semantics.selected
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.Dp
import ro.zach.language.R
import ro.zach.language.core.UnitProgressRecord
import ro.zach.language.core.UnitStatusRecord
import kotlin.math.roundToInt

private const val DISABLED_ALPHA: Float = 0.4f
private const val PARAGRAPH_SEPARATOR: String = "\n\n"

const val PAGE_HEADER_TITLE_TEST_TAG: String = "page-header-title"

/** Text in one of the type scale's styles. */
@Composable
fun LanguageText(
    text: String,
    style: TextStyle,
    modifier: Modifier = Modifier,
    color: Color = LanguageColors.ink,
    textAlign: TextAlign? = null,
    maxLines: Int = Int.MAX_VALUE,
) {
    val coloredStyle: TextStyle = if (textAlign == null) style.copy(color = color) else style.copy(color = color, textAlign = textAlign)

    BasicText(
        text = text,
        modifier = modifier,
        style = coloredStyle,
        maxLines = maxLines,
        overflow = if (maxLines == Int.MAX_VALUE) TextOverflow.Clip else TextOverflow.Ellipsis,
    )
}

/** An uppercase micro-label that carries structure, as the web's `.field-label` and captions do. */
@Composable
fun MicroLabel(text: String, modifier: Modifier = Modifier) {
    LanguageText(
        text = text.uppercase(),
        style = LanguageType.labelText.copy(letterSpacing = LanguageType.labelTracking),
        color = LanguageColors.inkMuted,
        modifier = modifier,
    )
}

@Composable
fun HairlineRule(modifier: Modifier = Modifier, color: Color = LanguageColors.rule) {
    Box(
        modifier = modifier
            .fillMaxWidth()
            .height(LanguageSpacing.hairline)
            .background(color),
    )
}

/** A single-stroke glyph tinted to `color`. */
@Composable
fun Glyph(glyph: GlyphKind, color: Color, modifier: Modifier = Modifier, size: Dp = LanguageSpacing.spaceMd) {
    Image(
        painter = painterResource(glyph.drawableResource()),
        contentDescription = null,
        colorFilter = ColorFilter.tint(color),
        modifier = modifier.size(size),
    )
}

enum class GlyphKind {
    Course,
    Review,
    Glossary,
    Progress,
    Settings,
    Play,
    Stop,
    Chevron,
    Back,
    Check,
    Pending;

    fun drawableResource(): Int {
        return when (this) {
            Course -> R.drawable.glyph_course
            Review -> R.drawable.glyph_review
            Glossary -> R.drawable.glyph_glossary
            Progress -> R.drawable.glyph_progress
            Settings -> R.drawable.glyph_settings
            Play -> R.drawable.glyph_play
            Stop -> R.drawable.glyph_stop
            Chevron -> R.drawable.glyph_chevron
            Back -> R.drawable.glyph_back
            Check -> R.drawable.glyph_check
            Pending -> R.drawable.glyph_pending
        }
    }
}

/** How a square button shows that what it started is still running. */
enum class SquareButtonPendingKind {
    NotPending,

    /** The label shows its own pending glyph; the button outlines it. */
    PendingWithinLabel,
    PendingInPlaceOfLabel,
}

/** A square, hairline-bordered button: ink by default, filled red when `isPrimary`. */
@Composable
fun SquareButton(
    onClick: () -> Unit,
    modifier: Modifier = Modifier,
    isPrimary: Boolean = false,
    isCompact: Boolean = false,
    pending: SquareButtonPendingKind = SquareButtonPendingKind.NotPending,
    enabled: Boolean = true,
    content: @Composable RowScope.(contentColor: Color) -> Unit,
) {
    val interactionSource: MutableInteractionSource = remember { MutableInteractionSource() }
    val isPressed: Boolean by interactionSource.collectIsPressedAsState()
    val contentColor: Color = squareButtonForeground(isPrimary, pending)
    val horizontalPadding: Dp = if (isCompact) LanguageSpacing.spaceSm else LanguageSpacing.spaceMd
    val verticalPadding: Dp = if (isCompact) LanguageSpacing.spaceXs else LanguageSpacing.spaceSm

    Row(
        modifier = modifier
            .alpha(if (enabled) 1f else DISABLED_ALPHA)
            .background(squareButtonBackground(isPrimary, isPressed))
            .border(LanguageSpacing.hairline, squareButtonBorder(isPrimary, pending))
            .clickable(interactionSource = interactionSource, indication = null, enabled = enabled, role = Role.Button, onClick = onClick)
            .padding(horizontal = horizontalPadding, vertical = verticalPadding),
        horizontalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceXs, Alignment.CenterHorizontally),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        PendingGlyphSwap(isIndicated = pending == SquareButtonPendingKind.PendingInPlaceOfLabel, glyphColor = contentColor) {
            Row(
                horizontalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceXs),
                verticalAlignment = Alignment.CenterVertically,
            ) {
                content(contentColor)
            }
        }
    }
}

/** A `SquareButton` labelled with body text. */
@Composable
fun SquareTextButton(
    text: String,
    onClick: () -> Unit,
    modifier: Modifier = Modifier,
    isPrimary: Boolean = false,
    isCompact: Boolean = false,
    pending: SquareButtonPendingKind = SquareButtonPendingKind.NotPending,
    enabled: Boolean = true,
) {
    SquareButton(onClick = onClick, modifier = modifier, isPrimary = isPrimary, isCompact = isCompact, pending = pending, enabled = enabled) { contentColor ->
        LanguageText(text = text, style = LanguageType.bodyText, color = contentColor)
    }
}

private fun squareButtonForeground(isPrimary: Boolean, pending: SquareButtonPendingKind): Color {
    if (isPrimary) {
        return LanguageColors.paper
    }

    return if (pending == SquareButtonPendingKind.PendingInPlaceOfLabel) LanguageColors.accentActive else LanguageColors.ink
}

private fun squareButtonBackground(isPrimary: Boolean, isPressed: Boolean): Color {
    if (isPrimary) {
        return if (isPressed) LanguageColors.accentHover else LanguageColors.accentActive
    }

    return if (isPressed) LanguageColors.rule else LanguageColors.paper
}

private fun squareButtonBorder(isPrimary: Boolean, pending: SquareButtonPendingKind): Color {
    if (isPrimary) {
        return LanguageColors.accentActive
    }

    return if (pending == SquareButtonPendingKind.NotPending) LanguageColors.ink else LanguageColors.accentActive
}

/** The state a choice button shows after the learner answers. */
enum class ChoiceMarkKind {
    Unmarked,
    Selected,
    Right,
    Wrong;

    fun foreground(): Color {
        return when (this) {
            Right -> LanguageColors.goodText
            Wrong -> LanguageColors.accentActive
            Unmarked, Selected -> LanguageColors.ink
        }
    }

    fun border(): Color {
        return when (this) {
            Unmarked -> LanguageColors.ink
            Selected -> LanguageColors.accentLink
            Right -> LanguageColors.goodText
            Wrong -> LanguageColors.accentActive
        }
    }

    fun borderWidth(): Dp {
        return if (this == Unmarked) LanguageSpacing.hairline else LanguageSpacing.selectedBorderWidth
    }
}

/** A full-width answer choice whose border and text show its mark. */
@Composable
fun ChoiceButton(
    mark: ChoiceMarkKind,
    onClick: () -> Unit,
    modifier: Modifier = Modifier,
    enabled: Boolean = true,
    content: @Composable RowScope.(contentColor: Color) -> Unit,
) {
    val interactionSource: MutableInteractionSource = remember { MutableInteractionSource() }
    val isPressed: Boolean by interactionSource.collectIsPressedAsState()

    Row(
        modifier = modifier
            .fillMaxWidth()
            .background(if (isPressed) LanguageColors.rule else LanguageColors.paper)
            .border(mark.borderWidth(), mark.border())
            .clickable(interactionSource = interactionSource, indication = null, enabled = enabled, role = Role.Button, onClick = onClick)
            .padding(LanguageSpacing.spaceSm + LanguageSpacing.spaceXs),
        horizontalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        content(mark.foreground())
    }
}

/** A small hairline-bordered tag; `isInk` fills it black, `isActive` outlines it red. */
@Composable
fun Tag(text: String, modifier: Modifier = Modifier, isInk: Boolean = false, isActive: Boolean = false) {
    val foreground: Color = when {
        isInk -> LanguageColors.paper
        isActive -> LanguageColors.accentActive
        else -> LanguageColors.ink
    }

    LanguageText(
        text = text.uppercase(),
        style = LanguageType.labelText.copy(letterSpacing = LanguageType.tagTracking),
        color = foreground,
        modifier = modifier
            .background(if (isInk) LanguageColors.ink else LanguageColors.paper)
            .border(LanguageSpacing.hairline, if (isActive) LanguageColors.accentActive else LanguageColors.ink)
            .padding(horizontal = LanguageSpacing.tagHorizontalPadding, vertical = LanguageSpacing.tagVerticalPadding),
    )
}

/** A hairline-bordered block that groups one dialogue, note, reading or session. */
fun Modifier.panel(): Modifier {
    return fillMaxWidth()
        .border(LanguageSpacing.hairline, LanguageColors.ink)
        .padding(LanguageSpacing.spaceMd)
}

/** A paper box with an ink outline and a solid ink block offset down and to the right behind it. */
fun Modifier.shadowedBox(shadowOffset: Dp = LanguageSpacing.boxShadowOffset): Modifier {
    return drawBehind {
        val offset: Float = shadowOffset.toPx()

        drawRect(color = LanguageColors.ink, topLeft = Offset(offset, offset), size = size)
    }
        .background(LanguageColors.paper)
        .border(LanguageSpacing.hairline, LanguageColors.ink)
}

/** Keeps prose to a readable measure on wide screens, centred. */
fun Modifier.readableWidth(): Modifier {
    return fillMaxWidth()
        .wrapContentWidth(Alignment.CenterHorizontally)
        .widthIn(max = LanguageSpacing.measure)
        .fillMaxWidth()
}

/** A square hairline progress bar, the web's `<progress>`. */
@Composable
fun SquareProgressBar(value: Int, total: Int, modifier: Modifier = Modifier) {
    val proportion: Float = if (total == 0) 0f else value.toFloat() / total.toFloat()

    Box(
        modifier = modifier
            .fillMaxWidth()
            .height(LanguageSpacing.progressBarHeight)
            .background(LanguageColors.paper)
            .border(LanguageSpacing.hairline, LanguageColors.ink),
    ) {
        Box(
            modifier = Modifier
                .fillMaxHeight()
                .fillMaxWidth(proportion)
                .background(LanguageColors.ink),
        )
    }
}

/** Splits prose on blank lines into paragraphs, as the web does. */
@Composable
fun Paragraphs(text: String, modifier: Modifier = Modifier) {
    val paragraphs: List<String> = text.split(PARAGRAPH_SEPARATOR)
        .map { paragraph -> paragraph.trim() }
        .filter { paragraph -> paragraph.isNotEmpty() }

    Column(modifier = modifier, verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm)) {
        for (paragraph in paragraphs) {
            LanguageText(text = paragraph, style = LanguageType.bodyText.copy(lineHeight = LanguageType.paragraphLineHeight))
        }
    }
}

/** A page title with an uppercase caption above it. */
@Composable
fun PageHeader(caption: String?, title: String, modifier: Modifier = Modifier) {
    Column(modifier = modifier.fillMaxWidth(), verticalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceXs)) {
        if (caption != null) {
            MicroLabel(text = caption)
        }

        LanguageText(text = title, style = LanguageType.headingText, modifier = Modifier.semantics { heading() }.testTag(PAGE_HEADER_TITLE_TEST_TAG))
    }
}

/** A secondary line of text centred in the space a page would fill. */
@Composable
fun StatusMessage(text: String, modifier: Modifier = Modifier) {
    Box(modifier = modifier.fillMaxSize().padding(LanguageSpacing.spaceMd), contentAlignment = Alignment.Center) {
        LanguageText(text = text, style = LanguageType.bodyText, color = LanguageColors.inkMuted, textAlign = TextAlign.Center)
    }
}

/** The statuses in the order a unit moves through them. */
val unitStatusRecords: List<UnitStatusRecord> = listOf(UnitStatusRecord.NOT_STARTED, UnitStatusRecord.IN_PROGRESS, UnitStatusRecord.COMPLETED)

fun UnitStatusRecord.label(): String {
    return when (this) {
        UnitStatusRecord.NOT_STARTED -> "Not started"
        UnitStatusRecord.IN_PROGRESS -> "In progress"
        UnitStatusRecord.COMPLETED -> "Completed"
    }
}

fun UnitStatusRecord.accessibilityName(): String {
    return when (this) {
        UnitStatusRecord.NOT_STARTED -> "not-started"
        UnitStatusRecord.IN_PROGRESS -> "in-progress"
        UnitStatusRecord.COMPLETED -> "completed"
    }
}

/** A unit's status, with its best score once it has been practised. */
@Composable
fun UnitStatusTag(unitProgress: UnitProgressRecord, modifier: Modifier = Modifier) {
    val hasAttempts: Boolean = unitProgress.attemptCount > 0u

    Row(modifier = modifier, horizontalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceXs), verticalAlignment = Alignment.CenterVertically) {
        when (unitProgress.status) {
            UnitStatusRecord.COMPLETED -> Tag(text = UnitStatusRecord.COMPLETED.label(), isInk = true)
            UnitStatusRecord.IN_PROGRESS -> Tag(text = UnitStatusRecord.IN_PROGRESS.label(), isActive = true)
            UnitStatusRecord.NOT_STARTED -> Tag(text = UnitStatusRecord.NOT_STARTED.label())
        }

        if (hasAttempts && unitProgress.status != UnitStatusRecord.NOT_STARTED) {
            LanguageText(text = "${(unitProgress.bestProportion * 100).roundToInt()}%", style = LanguageType.dataText, color = LanguageColors.inkMuted)
        }
    }
}

/** A hairline square with a check, labelled like the web's checkboxes. */
@Composable
fun SquareToggle(
    isOn: Boolean,
    onToggle: (Boolean) -> Unit,
    modifier: Modifier = Modifier,
    label: @Composable RowScope.() -> Unit,
) {
    Row(
        modifier = modifier.toggleable(value = isOn, interactionSource = null, indication = null, role = Role.Checkbox, onValueChange = onToggle),
        horizontalArrangement = Arrangement.spacedBy(LanguageSpacing.spaceSm),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        Box(
            modifier = Modifier
                .size(LanguageSpacing.checkBoxSize)
                .border(LanguageSpacing.hairline, LanguageColors.ink),
            contentAlignment = Alignment.Center,
        ) {
            if (isOn) {
                Glyph(glyph = GlyphKind.Check, color = LanguageColors.ink, size = LanguageSpacing.checkBoxSize - LanguageSpacing.spaceXs)
            }
        }

        label()
    }
}

/** A row of square hairline segments, one per option, with the chosen one filled ink. */
@Composable
fun <Option> SquareSegmentedControl(
    options: List<Option>,
    selection: Option,
    label: (Option) -> String,
    testTag: (Option) -> String,
    onSelect: (Option) -> Unit,
    modifier: Modifier = Modifier,
) {
    Row(modifier = modifier.fillMaxWidth()) {
        for (option in options) {
            val isSelected: Boolean = option == selection

            Box(
                modifier = Modifier
                    .weight(1f)
                    .background(if (isSelected) LanguageColors.ink else LanguageColors.paper)
                    .border(LanguageSpacing.hairline, LanguageColors.ink)
                    .clickable(interactionSource = null, indication = null, role = Role.Tab) { onSelect(option) }
                    .semantics { selected = isSelected }
                    .testTag(testTag(option))
                    .padding(horizontal = LanguageSpacing.spaceXs, vertical = LanguageSpacing.spaceSm),
                contentAlignment = Alignment.Center,
            ) {
                LanguageText(
                    text = label(option),
                    style = LanguageType.captionText,
                    color = if (isSelected) LanguageColors.paper else LanguageColors.ink,
                    maxLines = 1,
                )
            }
        }
    }
}
