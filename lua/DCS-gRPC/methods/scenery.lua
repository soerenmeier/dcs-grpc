--
-- RPC unit actions
-- https://wiki.hoggitworld.com/view/DCS_Class_Scenery
--

GRPC.methods.sceneryDestroy = function(params)
  local zone = trigger.misc.getZone(params.searchZone)

  world.searchObjects(
    Object.Category.SCENERY,
    {
      id = world.VolumeType.SPHERE,
      params = {
        point = zone.point,
        radius = zone.radius
      }
    },
    function(foundItem, val)
      -- this should be a scenery object
      local name = tostring(foundItem:getName())

      GRPC.logInfo("sceneryDestroy name: " .. name)

      for _, v in pairs(params.objectIds) do
        if v == name then
          foundItem:destroy()
          return
        end
      end
    end
  )

  return GRPC.success({})
end
