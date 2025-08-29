/**
 * Intro Animation System
 * Handles triggering and managing intro animations for components
 */

class IntroAnimationSystem {
    constructor() {
        this.observers = new Map();
        this.animatedElements = new Set();
        this.debugMode = false;
        this.init();
    }

    init() {
        // Initialize on DOM ready
        if (document.readyState === 'loading') {
            document.addEventListener('DOMContentLoaded', () => this.setupAnimations());
        } else {
            this.setupAnimations();
        }

        // Re-setup animations when new content is added (for SPA)
        this.setupMutationObserver();
    }

    setupAnimations() {
        // Find all elements with intro animation classes
        const animatedElements = document.querySelectorAll('[class*="intro-"]');
        
        animatedElements.forEach(element => {
            this.setupElementAnimation(element);
        });
    }

    setupElementAnimation(element) {
        // Skip if already processed
        if (this.animatedElements.has(element)) {
            return;
        }

        const animationType = this.getAnimationType(element);
        const trigger = this.getAnimationTrigger(element);
        const duration = this.getAnimationDuration(element);
        const delay = this.getAnimationDelay(element);
        const easing = this.getAnimationEasing(element);
        const offset = this.getAnimationOffset(element);
        const repeat = this.getAnimationRepeat(element);

        // Apply animation properties
        this.applyAnimationProperties(element, duration, delay, easing);

        // Set up trigger
        switch (trigger) {
            case 'load':
                this.triggerLoadAnimation(element, delay);
                break;
            case 'scroll':
                this.setupScrollTrigger(element, offset, repeat);
                break;
            case 'hover':
                this.setupHoverTrigger(element);
                break;
            case 'click':
                this.setupClickTrigger(element);
                break;
        }

        this.animatedElements.add(element);

        if (this.debugMode) {
            element.setAttribute('data-animation-type', animationType);
        }
    }

    getAnimationType(element) {
        const classes = element.className.split(' ');
        for (const cls of classes) {
            if (cls.startsWith('intro-') && !cls.includes('duration') && !cls.includes('delay') && !cls.includes('ease')) {
                return cls.replace('intro-', '');
            }
        }
        return 'fade-in';
    }

    getAnimationTrigger(element) {
        return element.dataset.introTrigger || 'scroll';
    }

    getAnimationDuration(element) {
        return element.dataset.introDuration || '0.6s';
    }

    getAnimationDelay(element) {
        return element.dataset.introDelay || '0s';
    }

    getAnimationEasing(element) {
        return element.dataset.introEasing || 'ease-out';
    }

    getAnimationOffset(element) {
        return element.dataset.introOffset || '100px';
    }

    getAnimationRepeat(element) {
        return element.dataset.introRepeat === 'true';
    }

    applyAnimationProperties(element, duration, delay, easing) {
        element.style.transitionDuration = duration;
        element.style.transitionDelay = delay;
        element.style.transitionTimingFunction = easing;
        element.style.animationDuration = duration;
        element.style.animationDelay = delay;
        element.style.animationTimingFunction = easing;
    }

    triggerLoadAnimation(element, delay) {
        const delayMs = this.parseTime(delay);
        setTimeout(() => {
            element.classList.add('animate');
        }, delayMs);
    }

    setupScrollTrigger(element, offset, repeat) {
        const offsetPx = parseInt(offset) || 100;
        
        const observer = new IntersectionObserver((entries) => {
            entries.forEach(entry => {
                if (entry.isIntersecting) {
                    entry.target.classList.add('animate');
                    
                    if (!repeat) {
                        observer.unobserve(entry.target);
                    }
                } else if (repeat) {
                    entry.target.classList.remove('animate');
                }
            });
        }, {
            rootMargin: `0px 0px -${offsetPx}px 0px`,
            threshold: 0.1
        });

        observer.observe(element);
        this.observers.set(element, observer);
    }

    setupHoverTrigger(element) {
        element.addEventListener('mouseenter', () => {
            element.classList.add('animate');
        });

        element.addEventListener('mouseleave', () => {
            if (this.getAnimationRepeat(element)) {
                element.classList.remove('animate');
            }
        });
    }

    setupClickTrigger(element) {
        element.addEventListener('click', () => {
            element.classList.add('animate');
            
            if (this.getAnimationRepeat(element)) {
                setTimeout(() => {
                    element.classList.remove('animate');
                }, this.parseTime(this.getAnimationDuration(element)) + 100);
            }
        });
    }

    setupMutationObserver() {
        const observer = new MutationObserver((mutations) => {
            mutations.forEach(mutation => {
                mutation.addedNodes.forEach(node => {
                    if (node.nodeType === Node.ELEMENT_NODE) {
                        // Check if the added node has animation classes
                        if (node.className && node.className.includes('intro-')) {
                            this.setupElementAnimation(node);
                        }
                        
                        // Check child elements
                        const animatedChildren = node.querySelectorAll && node.querySelectorAll('[class*="intro-"]');
                        if (animatedChildren) {
                            animatedChildren.forEach(child => {
                                this.setupElementAnimation(child);
                            });
                        }
                    }
                });
            });
        });

        observer.observe(document.body, {
            childList: true,
            subtree: true
        });
    }

    parseTime(timeString) {
        const value = parseFloat(timeString);
        if (timeString.includes('ms')) {
            return value;
        } else {
            return value * 1000; // Convert seconds to milliseconds
        }
    }

    // Public API methods
    animateElement(element, animationType = 'fade-in', options = {}) {
        const {
            duration = '0.6s',
            delay = '0s',
            easing = 'ease-out',
            trigger = 'immediate'
        } = options;

        // Add animation class
        element.classList.add(`intro-${animationType}`);
        
        // Apply properties
        this.applyAnimationProperties(element, duration, delay, easing);

        if (trigger === 'immediate') {
            const delayMs = this.parseTime(delay);
            setTimeout(() => {
                element.classList.add('animate');
            }, delayMs);
        } else {
            // Set up trigger
            element.dataset.introTrigger = trigger;
            this.setupElementAnimation(element);
        }
    }

    resetElement(element) {
        element.classList.remove('animate');
        
        // Remove from tracking
        this.animatedElements.delete(element);
        
        // Clean up observer
        if (this.observers.has(element)) {
            this.observers.get(element).disconnect();
            this.observers.delete(element);
        }
    }

    enableDebugMode() {
        this.debugMode = true;
        document.body.classList.add('intro-debug');
    }

    disableDebugMode() {
        this.debugMode = false;
        document.body.classList.remove('intro-debug');
    }

    // Stagger animation for multiple elements
    staggerElements(elements, options = {}) {
        const {
            staggerDelay = 0.1,
            animationType = 'fade-in-up',
            trigger = 'scroll'
        } = options;

        elements.forEach((element, index) => {
            element.classList.add(`intro-${animationType}`);
            element.style.setProperty('--stagger-index', index);
            element.style.setProperty('--stagger-delay', `${staggerDelay}s`);
            element.dataset.introTrigger = trigger;
            
            this.setupElementAnimation(element);
        });
    }

    // Cleanup method
    destroy() {
        this.observers.forEach(observer => observer.disconnect());
        this.observers.clear();
        this.animatedElements.clear();
    }
}

// Global instance
window.IntroAnimationSystem = new IntroAnimationSystem();

// Utility functions for Rust/WASM integration
window.setupIntroAnimation = function(elementId, animationType, options = {}) {
    const element = document.getElementById(elementId);
    if (element) {
        window.IntroAnimationSystem.animateElement(element, animationType, options);
    }
};

window.setupIntroAnimationByClass = function(className, animationType, options = {}) {
    const elements = document.querySelectorAll(`.${className}`);
    elements.forEach(element => {
        window.IntroAnimationSystem.animateElement(element, animationType, options);
    });
};

window.staggerIntroAnimations = function(selector, options = {}) {
    const elements = document.querySelectorAll(selector);
    window.IntroAnimationSystem.staggerElements(Array.from(elements), options);
};

window.resetIntroAnimation = function(elementId) {
    const element = document.getElementById(elementId);
    if (element) {
        window.IntroAnimationSystem.resetElement(element);
    }
};

// Export for module systems
if (typeof module !== 'undefined' && module.exports) {
    module.exports = IntroAnimationSystem;
}
