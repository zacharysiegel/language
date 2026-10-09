import SwiftUI

/// The bounds of the control a floating tip opens below; a page has at most one.
struct FloatingTipAnchorKey: PreferenceKey {
    static let defaultValue: Anchor<CGRect>? = nil

    static func reduce(value: inout Anchor<CGRect>?, nextValue: () -> Anchor<CGRect>?) {
        value = value ?? nextValue()
    }
}

/// Draws a tip below the floating tip anchor, above the content and without moving it, while `isShown`.
/// A press anywhere outside the tip dismisses it without reaching the content, as does `autoCloseDuration` passing.
struct FloatingTipModifier<Tip: View>: ViewModifier {
    let isShown: Bool
    let autoCloseDuration: Duration
    let onDismiss: () -> Void
    @ViewBuilder let tip: () -> Tip

    func body(content: Content) -> some View {
        content
            .overlayPreferenceValue(FloatingTipAnchorKey.self) { anchor in
                if isShown, let anchor {
                    GeometryReader { proxy in
                        layer(anchorBounds: proxy[anchor], layerSize: proxy.size)
                    }
                }
            }
            .task(id: isShown) {
                await dismissAfterAutoCloseDuration()
            }
    }

    private func layer(anchorBounds: CGRect, layerSize: CGSize) -> some View {
        let tipWidth: CGFloat = min(.floatingTipMaximumWidth, layerSize.width - 2 * .spaceMd - .boxShadowOffset)
        let rightmostTipLeading: CGFloat = layerSize.width - .spaceMd - .boxShadowOffset - tipWidth
        let tipLeading: CGFloat = max(.spaceMd, min(anchorBounds.minX, rightmostTipLeading))

        return ZStack(alignment: .topLeading) {
            Color.clear
                .contentShape(Rectangle())
                .gesture(DragGesture(minimumDistance: 0).onChanged { _ in onDismiss() })

            tip()
                .frame(width: tipWidth, alignment: .leading)
                .contentShape(Rectangle())
                .offset(x: tipLeading, y: anchorBounds.maxY + .spaceXs)
        }
    }

    private func dismissAfterAutoCloseDuration() async {
        guard isShown else {
            return
        }

        do {
            try await Task.sleep(for: autoCloseDuration)
        } catch {
            return
        }

        onDismiss()
    }
}

extension View {
    func floatingTipAnchor() -> some View {
        anchorPreference(key: FloatingTipAnchorKey.self, value: .bounds) { anchor in anchor }
    }

    func floatingTip<Tip: View>(
        isShown: Bool,
        autoCloseDuration: Duration,
        onDismiss: @escaping () -> Void,
        @ViewBuilder tip: @escaping () -> Tip,
    ) -> some View {
        modifier(FloatingTipModifier(isShown: isShown, autoCloseDuration: autoCloseDuration, onDismiss: onDismiss, tip: tip))
    }
}
