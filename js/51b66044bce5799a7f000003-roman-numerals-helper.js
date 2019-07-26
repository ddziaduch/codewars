const map = {
  'M': 1000,
  'CM': 900,
  'D': 500,
  'CD': 400,
  'C': 100,
  'XC': 90,
  'L': 50,
  'XL': 40,
  'X': 10,
  'IX': 9,
  'V': 5,
  'IV': 4,
  'I': 1,
}

const RomanNumerals = {
  /**
   * @param {number} number
   * @returns {string}
   */
  toRoman(number) {
    let numeral = ''
    while (number >= 1) {
      Object.entries(map).forEach(([romeDigit, arabicDigit]) => {
        const numberOfRomeDigits = Math.floor(number / arabicDigit)
        if (numberOfRomeDigits > 0) {
          numeral += String(romeDigit).repeat(numberOfRomeDigits)
          number -= arabicDigit * numberOfRomeDigits
        }
      })
    }
    return numeral
  },

  /**
   * @param {string} numeral
   * @returns {number}
   */
  fromRoman(numeral) {
    let number = 0;

    while (numeral.length > 0) {
      let segment = numeral.substring(0, 2)
      if (segment.length === 2 && !map.hasOwnProperty(segment)) {
        segment = numeral.substring(0, 1)
      }
      number += map[segment]
      numeral = numeral.substring(segment.length);
    }

    return number;
  },
}

console.assert(RomanNumerals.toRoman(1000) === 'M')
console.assert(RomanNumerals.toRoman(999) === "CMXCIX")
console.assert(RomanNumerals.toRoman(4) === 'IV')
console.assert(RomanNumerals.toRoman(1) === 'I')
console.assert(RomanNumerals.toRoman(1991) === 'MCMXCI')
console.assert(RomanNumerals.toRoman(2006) === 'MMVI')
console.assert(RomanNumerals.toRoman(2020) === 'MMXX')

console.assert(RomanNumerals.fromRoman('XXI') === 21)
console.assert(RomanNumerals.fromRoman('I') === 1)
console.assert(RomanNumerals.fromRoman('III') === 3)
console.assert(RomanNumerals.fromRoman('IV') === 4)
console.assert(RomanNumerals.fromRoman('MMVII') === 2007)
console.assert(RomanNumerals.fromRoman('MDCLXIX') === 1669)