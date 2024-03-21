declare global {
	namespace App {}

	interface ViewTransition {
		updateCallbackDone: Promise<void>;
		ready: Promise<void>;
		finished: Promise<void>;
		skipTransition: () => void;
	}

	interface Document {
		startViewTransition(updateCallback: () => Promise<void>): ViewTransition;
	}
}

export {};
