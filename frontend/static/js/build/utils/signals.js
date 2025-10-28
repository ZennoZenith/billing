export class Signal {
    _value;
    subscribers = new Set();
    constructor(initialValue) {
        this._value = initialValue;
    }
    get value() {
        return this._value;
    }
    set value(newValue) {
        if (newValue !== this._value) {
            // Only notify if the value actually changed
            this._value = newValue;
            this.notifySubscribers();
        }
    }
    subscribe(subscriber) {
        this.subscribers.add(subscriber);
    }
    unsubscribe(subscriber) {
        this.subscribers.delete(subscriber);
    }
    notifySubscribers() {
        this.subscribers.forEach((subscriber) => subscriber(this._value));
    }
}
// // Usage example:
// const mySignal = new Signal<number>(0);
// mySignal.subscribe((newValue) => {
//   console.log(`Signal value changed to: ${newValue}`);
// });
// mySignal.value = 10; // This will log: "Signal value changed to: 10"
// mySignal.value = 10; // This will not log anything as the value didn't change
// mySignal.value = 20; // This will log: "Signal value changed to: 20"
