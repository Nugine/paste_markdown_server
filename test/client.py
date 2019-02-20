import requests as rq

payloads = [
    {"title": "a", "author": "b"},
    {"title": "a", "author": "b", "content": "c"}
]

url = 'http://localhost:8080/post'

for p in payloads:
    resp = rq.post(url, json=p)
    print(resp)
    print(resp.text)
    if resp.status_code == 200:
        location = resp.json()['location']
        post_url = 'http://localhost:8080/post/'+location
        resp = rq.get(post_url)
        print(resp.text)
