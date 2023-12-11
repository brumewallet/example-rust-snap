import { test } from "@hazae41/phobos"
import { Snap, initBundledOnce } from "index.js"

await test("test", async () => {
  await initBundledOnce()

  const snap = new Snap({
    log(text: string) {
      console.log("[snap]", text)
    }
  })

  function request(data: unknown) {
    return JSON.parse(snap.on_request(JSON.stringify(data)))
  }

  const response = request({
    method: "hello",
    params: ["world"]
  })

  console.log(response)
})