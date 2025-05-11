import { useEffect, useState } from 'react'
import init from '../wasm/echo_frame_engine'

const promise = init()

export function useWasmLoading() {
    const [isLoading, setIsLoading] = useState(true)

    useEffect(() => {
        promise.then(() => {
            setIsLoading(false)
        })
    }, [])

    return isLoading
}

/**
 * @param {() => T} callback
 * @returns {[T | null, boolean]}
 * @template T
 */
export function useWasm(callback) {
    const isLoading = useWasmLoading()

    if (isLoading) {
        return [null, isLoading]
    }

    const result = callback()
    return [result, isLoading]
}
