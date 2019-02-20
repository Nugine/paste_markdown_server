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
