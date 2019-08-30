// https://www.codewars.com/kata/513e7e1aee7d36073e00000d/train/javascript

Array.prototype.each = function (callback) {
	for (let i in this) {
		if (callback(this[i], i, this) === true) {
			return
		}
	}
}
