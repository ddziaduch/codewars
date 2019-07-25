function namespace(root, path, value) {
  if (value === undefined) {
    return getNamespace(root, path)
  } else {
    setNamespace(root, path, value)
  }
}

const getNamespace = (root, path) => {
  try {
    return eval('root.' + path)
  } catch (error) {
    return undefined
  }
}

const setNamespace = (root, path, value) => {
  const parts = path.split('.')
  let currentRoot = root
  let currentPart = ''
  while (parts.length > 0) {
    currentPart = parts.shift()
    if (currentRoot[currentPart] === undefined) {
      currentRoot[currentPart] = {}
    }
    currentRoot = currentRoot[currentPart]
  }
  eval('root.' + path + ' = value')
}
