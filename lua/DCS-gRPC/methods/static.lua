--
-- RPC unit actions
-- https://wiki.hoggitworld.com/view/DCS_Class_Static_Object
--

GRPC.methods.staticDestroy = function(params)
  local obj = StaticObject.getByName(params.name)
  if obj == nil then
    return GRPC.errorNotFound("static `" .. tostring(params.name) .. "` does not exist")
  end

  obj:destroy()
  return GRPC.success({})
end
