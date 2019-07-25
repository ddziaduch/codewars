const extractIds = (data) => {
  if (!data.id) {
    return []
  }
  let results = [data.id, ...data.items]
  let hasArrayObjects = true;
  while (hasArrayObjects) {
    hasArrayObjects = results.some((item, index) => {
      if (item instanceof Object) {
        const chunk = []
        if (item.id) {
          chunk.push(item.id)
        }
        if (item.items) {
          chunk.push(...item.items)
        }
        results.splice(index, 1, ...chunk)
        return true;
      }
      return false;
    })
  }
  return results
}

const data = {
  id: 1, items: [
    { id: 2 }, {
      id: 3, items: [
        { id: 4 },
      ],
    },
  ],
}

const results = extractIds(data)
console.assert(results.length === 4, 'results.length === 4')
console.assert(results[0] === 1, 'results[0] === 1')
console.assert(results[1] === 2, 'results[1] === 2')
console.assert(results[2] === 3, 'results[2] === 3')
console.assert(results[3] === 4, 'results[3] === 4');
