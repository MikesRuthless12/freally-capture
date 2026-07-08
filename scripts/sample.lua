-- Freally Capture sample script (TASK-703).
--
-- Reacts to going live: switches the program to the scene named "Live"
-- (if one exists) and logs what happened. Enable it via the Scripts dialog.
--
-- The fcap API:
--   fcap.on(event, handler)      -- streamStarted / streamEnded /
--                                -- recordingStarted / recordingStopped /
--                                -- sceneChanged { scene } / state { ... }
--   fcap.command(name, params)   -- the same allowlist as the remote API
--   fcap.log(...)                -- into the app log

fcap.on("streamStarted", function()
  fcap.log("went live — switching to the Live scene")
  local ok, err = pcall(function()
    fcap.command("setProgramScene", { scene = "Live" })
  end)
  if not ok then
    fcap.log("no Live scene to switch to: " .. tostring(err))
  end
end)

fcap.on("sceneChanged", function(data)
  fcap.log("program scene is now " .. tostring(data.scene))
end)
