class PaginationHelper {
  constructor(collection, itemsPerPage) {
    this.collection = collection
    this.itemsPerPage = itemsPerPage
  }

  itemCount() {
    return this.collection.length
  }

  pageCount() {
    return Math.ceil(this.itemCount() / this.itemsPerPage)
  }

  pageItemCount(pageIndex) {
    const pageNumber = pageIndex + 1
    const pageCount = this.pageCount()

    if (pageNumber < pageCount && pageNumber > 0) {
      return this.itemsPerPage
    }

    if (pageNumber === pageCount) {
      return this.itemCount() - ((pageCount - 1) * this.itemsPerPage)
    }

    return -1
  }

  pageIndex(itemIndex) {
    const itemCount = this.itemCount()
    const itemNumber = itemIndex + 1

    if (itemCount > 0 && itemNumber >= 0 && itemNumber <= itemCount) {
      const pageNumber = Math.ceil(itemNumber / this.itemsPerPage)
      return pageNumber - 1
    }

    return -1
  }
}

const helper = new PaginationHelper(['a', 'b', 'c', 'd', 'e', 'f'], 4)
console.assert(helper.pageCount() === 2, 'helper.pageCount() === 2') //should == 2
console.assert(helper.itemCount() === 6, 'helper.itemCount() === 6') //should == 6
console.assert(helper.pageItemCount(0) === 4, 'helper.pageItemCount(0) === 4') //should == 4
console.assert(helper.pageItemCount(1) === 2, 'helper.pageItemCount(1) === 2') // last page - should == 2
console.assert(helper.pageItemCount(2) === -1, 'helper.pageItemCount(2) === -1') // should == -1 since the page is invalid

// pageIndex takes an item index and returns the page that it belongs on
console.assert(helper.pageIndex(5) === 1) //should == 1 (zero based index)
console.assert(helper.pageIndex(2) === 0) //should == 0
console.assert(helper.pageIndex(20) === -1) //should == -1
console.assert(helper.pageIndex(-10) === -1) //should == -1