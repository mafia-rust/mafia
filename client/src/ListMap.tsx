export type ListMapData<K, V> = [K, V][]

export default class ListMap<K, V> {
    list: ListMapData<K, V>;
    constructor(list: ListMapData<K, V> = []) {
        this.list = list
    }
    get(key: K): V | null {
        for (const [k, v] of this.list) {
            if (k === key) {
                return v
            }
        }
        return null
    }
    set(key: K, value: V) {
        this.list = this.list.filter(([k, v]) => k !== key);
        this.list.push([key, value])
    }
    delete(key: K) {
        this.list = this.list.filter(([k, v]) => k !== key)
    }
    entries(): ListMapData<K, V> {
        return this.list
    }
    keys(): K[] {
        return this.list.map(([k, v]) => k)
    }
    values(): V[] {
        return this.list.map(([k, v]) => v)
    }
}