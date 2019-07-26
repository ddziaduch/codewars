const normalizeParams = (params, paramsToStrip) => {
    const normalized = params.split('&').reduce((accumulator, param) => {
        const [key, value] = param.split('=')
        if (!paramsToStrip.includes(key) && !accumulator.hasOwnProperty(key)) {
            accumulator[key] = value
        }
        return accumulator
    }, {})
    return Object.entries(normalized).map(entry => `${entry[0]}=${entry[1]}`).join('&')
}

const stripUrlParams = (url, paramsToStrip = []) => {
    const [prefix, params] = url.split('?')
    if (!params) return url
    const normalizedParams = normalizeParams(params, paramsToStrip)
    return `${prefix}?${normalizedParams}`
}
