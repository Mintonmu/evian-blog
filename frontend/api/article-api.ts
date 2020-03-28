import { ENDPOINT } from '../utils/config'
import { Article, toArticle, toArticles } from '../interfaces'

import fetch from 'node-fetch'

async function getArticles(): Promise<Article[]> {
    const res = await fetch(ENDPOINT + '/api/v1/articles');
    if (res.status !== 200) {
        return Promise.reject(res.statusText);
    }
    const data = await toArticles(res);
    return Promise.resolve(data);
}

async function getArticle(articleTitle: string): Promise<Article> {
    const res = await fetch(ENDPOINT + '/api/v1/article/' + articleTitle);
    if (res.status != 200) {
        return Promise.reject(res.statusText);
    }
    const data = await toArticle(res);
    return Promise.resolve(data);
}

export { getArticle, getArticles };
