<p align="center"><img src="https://user-images.githubusercontent.com/19860968/217567579-df6f98b9-addf-4cb4-9650-0038fd8f347e.svg" width="120"></p>
<hr />

# rush

An implementation of a browser
Inspired by https://github.com/servo/servo

These are the parts currently planned.

- [ ] Network Layer
- [ ] HTML Parser
- [ ] CSS Parser
- [ ] Render Engine
- [ ] Javascript Engine
- [ ] User Interface

## Architecture


                  ┌──────────────────┐
                  │  User Interface  │
                  └────────┬─────────┘
                           │
                           ▼
                  ┌──────────────────┐
                  │  Browser engine  │
                  │  ( not planned ) │
                  └────────┬─────────┘
                           │
                           ▼
           ┌──────────────────────────────────┐
           │          Render engine           │
           │                                  │
           │ ┌─────────────┐  ┌─────────────┐ │
           │ │ CSS Parser  │  │ HTML Parser │ │
           │ └─────────────┘  └─────────────┘ │
           └───────┬─────────────────┬────────┘
                   │                 │
                   ▼                 ▼
            ┌────────────┐    ┌──────────────┐
            │   Network  │    │  JS Engine   │
            └────────────┘    └──────────────┘

