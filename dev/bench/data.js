window.BENCHMARK_DATA = {
  "lastUpdate": 1701046584157,
  "repoUrl": "https://github.com/Javagedes/edk2_parser",
  "entries": {
    "Rust Benchmark": [
      {
        "commit": {
          "author": {
            "name": "Javagedes",
            "username": "Javagedes"
          },
          "committer": {
            "name": "Javagedes",
            "username": "Javagedes"
          },
          "id": "29f9daea0b9e27c2a2a2fe4ab910c29267aa7941",
          "message": "add benchmarking to github actions",
          "timestamp": "2023-11-25T20:56:46Z",
          "url": "https://github.com/Javagedes/edk2_parser/pull/3/commits/29f9daea0b9e27c2a2a2fe4ab910c29267aa7941"
        },
        "date": 1701046583654,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse_inf/baseLib",
            "value": 1916863,
            "range": "± 83582",
            "unit": "ns/iter"
          },
          {
            "name": "parse_inf/opensslLib",
            "value": 2769841,
            "range": "± 45982",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}