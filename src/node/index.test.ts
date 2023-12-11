import { test } from "@hazae41/phobos"
import { Snap, initBundledOnce } from "index.js"

await test("test", async () => {
  await initBundledOnce()

  const snap = new Snap({
    log(text: string) {
      console.log(text)
    },
    request(request: string) {
      const req = JSON.parse(request)

      if (req.method === "snap_log") {
        console.log(req.params[0])
        return JSON.stringify({})
      }

      return JSON.stringify({
        error: {
          message: "Unknown method"
        }
      })
    }
  })

  function request(data: unknown) {
    return JSON.parse(snap.on_request(JSON.stringify(data)))
  }

  const response = request({
    method: "echo",
    params: ["Hello world!"]
  })

  console.log(response)
})