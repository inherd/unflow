
[Sushi: A DSL for Conversational Flow Design](https://techlab.bol.com/sushi/)

GitHub: [https://github.com/dorost/sushi](https://github.com/dorost/sushi)

```
val flows = mutableListOf(
    Action().apply {
        name = "go to the store"
        id = "first-action-id"
        type = "go-to-store"
        source = true
        nextBlocks = mutableListOf("second-action-id")
    }, Action().apply {
        name = "ask the menu from waiteress"
        id = "second-action-id"
        type = "ask-menu"
        nextBlocks = mutableListOf("third-action-id")
    }, Branch().apply {
        name = "check if they have Maguro Teryaki"
        id = "check-maguro"
        on = "menu"
        mapping = mapOf(
            ""
        )
    }
)

flowEngine.wire(flows)
flowEngine.executeFlow()
flowEngine.await()
```