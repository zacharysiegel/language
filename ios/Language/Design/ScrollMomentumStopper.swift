import SwiftUI
import UIKit

/// Stops the momentum scrolling of the UIKit scroll view behind a SwiftUI `ScrollView`.
@MainActor
final class ScrollMomentumStopper {
    private weak var scrollView: UIScrollView?

    func attach(to scrollView: UIScrollView) {
        self.scrollView = scrollView
    }

    func stopMomentum() {
        guard let scrollView, scrollView.isDecelerating else {
            return
        }

        /* Ends the deceleration, which assigning contentOffset does not. */
        scrollView.setContentOffset(scrollView.contentOffset, animated: false)
    }
}

extension View {
    /// Attaches `stopper` to the scroll view whose content this view is part of.
    func scrollMomentumStopper(_ stopper: ScrollMomentumStopper) -> some View {
        background(EnclosingScrollViewFinder(stopper: stopper))
    }
}

private struct EnclosingScrollViewFinder: UIViewRepresentable {
    let stopper: ScrollMomentumStopper

    func makeUIView(context: Context) -> EnclosingScrollViewFinderView {
        EnclosingScrollViewFinderView(stopper: stopper)
    }

    func updateUIView(_ uiView: EnclosingScrollViewFinderView, context: Context) {}
}

private final class EnclosingScrollViewFinderView: UIView {
    private let stopper: ScrollMomentumStopper

    init(stopper: ScrollMomentumStopper) {
        self.stopper = stopper
        super.init(frame: .zero)
        isUserInteractionEnabled = false
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) is not used")
    }

    override func didMoveToWindow() {
        super.didMoveToWindow()

        guard window != nil, let scrollView = enclosingScrollView() else {
            return
        }

        stopper.attach(to: scrollView)
    }

    private func enclosingScrollView() -> UIScrollView? {
        var ancestor: UIView? = superview

        while let view = ancestor {
            if let scrollView = view as? UIScrollView {
                return scrollView
            }

            ancestor = view.superview
        }

        return nil
    }
}
