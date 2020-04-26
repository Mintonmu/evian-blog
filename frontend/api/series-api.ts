import { ENDPOINT } from '../utils/config'
import { Series, ArticleMeta } from '../interfaces'

import fetch from 'node-fetch'

async function getSeries(): Promise<Series[]> {
    const res = await fetch(ENDPOINT + '/api/v1/series');
    if (res.status !== 200) {
        return Promise.reject(res.statusText);
    }
    const data = await res.json();
    return Promise.resolve(data);
}

async function getArticlesCountOfSeries(seriesName: string): Promise<number> {
    const res = await fetch(ENDPOINT + `/api/v1/series/${seriesName}/count`);
    if (res.status !== 200) {
        return Promise.reject(res.statusText);
    }
    const data = await res.json();
    return Promise.resolve(data);
}

async function getArticlesOfSeries(seriesName: string, pageIndex: number, pageSize: number): Promise<ArticleMeta[]> {
    const res = await fetch(ENDPOINT + `/api/v1/series/${seriesName}?pageIndex=${pageIndex}&pageSize=${pageSize}`);
    if (res.status !== 200) {
        return Promise.reject(res.statusText);
    }
    const data = await res.json();
    return Promise.resolve(data);
}

export { getSeries, getArticlesCountOfSeries, getArticlesOfSeries };