#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

const PI = 3.14159265358979323846264338327950288;

struct CustomMaterial {
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material: CustomMaterial;
@group(1) @binding(1)
var<uniform> luminosity: f32;



struct FragmentInput {
  @builtin(front_facing) is_front: bool,
  @builtin(position) frag_coord: vec4<f32>,
  #import bevy_pbr::mesh_vertex_output
}

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
  var N = normalize(in.world_normal);
  var V = normalize(view.world_position.xyz - in.world_position.xyz);
  let NdotV = max(dot(N,V), 0.0000000001);
  let glow = pow(NdotV, .2); 
  let refl = reflect(-V, N);
  let noise_level_1 = snoise_vf4_(vec4(refl * 15.0, globals.time*0.1));
  let noise_level_2 = snoise_vf4_(vec4(refl, globals.time*0.05 + 100.));
  let noise = clamp(noise_level_1*0.8 + noise_level_2*0.8, -1.0, 1.0);
  let alpha = pow((noise + 1.1)/2.0, 0.3);

  // TODO load from rust
  let brightness = 7.0;
  //var col = vec3(0.0,0.4,1.0) * brightness;
  var col = material.color.xyz * luminosity;
  col = mix(vec3(0.0,0.0,0.0), col*alpha, glow);

  return vec4(col, 1.0);
}















fn mod289(x: vec4<f32>) -> vec4<f32> {
  return x - floor(x * (1.0 / 289.0)) * 289.0f;
}

fn mod289_f(x : f32) -> f32 {
  return x - floor(x * (1.0 / 289.0)) * 289.0f;
}

fn permute_f1_(x_3 : ptr<function, f32>) -> f32 {
  var param_1 : f32;
  let x_69 : f32 = *(x_3);
  let x_72 : f32 = *(x_3);
  param_1 = (((x_69 * 34.0f) + 1.0f) * x_72);
  return mod289_f(param_1);
}

fn permute_vf4_(x_2 : ptr<function, vec4<f32>>) -> vec4<f32> {
  var param : vec4<f32>;
  let x_57 : vec4<f32> = *(x_2);
  let x_63 : vec4<f32> = *(x_2);
  param = (((x_57 * 34.0f) + vec4<f32>(1.0f, 1.0f, 1.0f, 1.0f)) * x_63);
  let x_66 : vec4<f32> = mod289(param);
  return x_66;
}

fn grad4_f1_vf4_(j : ptr<function, f32>, ip : ptr<function, vec4<f32>>) -> vec4<f32> {
  var p : vec4<f32>;
  var s : vec4<f32>;
  let x_92 : f32 = *(j);
  let x_95 : vec4<f32> = *(ip);
  let x_105 : f32 = (*(ip)).z;
  let x_108 : vec3<f32> = ((floor((fract((vec3<f32>(x_92, x_92, x_92) * vec3<f32>(x_95.x, x_95.y, x_95.z))) * 7.0f)) * x_105) - vec3<f32>(1.0f, 1.0f, 1.0f));
  let x_109 : vec4<f32> = p;
  p = vec4<f32>(x_108.x, x_108.y, x_108.z, x_109.w);
  let x_112 : vec4<f32> = p;
  p.w = (1.5f - dot(abs(vec3<f32>(x_112.x, x_112.y, x_112.z)), vec3<f32>(1.0f, 1.0f, 1.0f)));
  let x_121 : vec4<f32> = p;
  s = select(vec4<f32>(0.0f, 0.0f, 0.0f, 0.0f), vec4<f32>(1.0f, 1.0f, 1.0f, 1.0f), (x_121 < vec4<f32>(0.0f, 0.0f, 0.0f, 0.0f)));
  let x_140 : vec3<f32> = p.xyz + (s.xyz * 2.0f - vec3<f32>(1.0f)) * s.www;
  p = vec4<f32>(x_140.x, x_140.y, x_140.z, p.w);
  return p;
}

fn taylorInvSqrt_vf4_(r : ptr<function, vec4<f32>>) -> vec4<f32> {
  let x_80 : vec4<f32> = *(r);
  return (vec4<f32>(1.792842864990234375f, 1.792842864990234375f, 1.792842864990234375f, 1.792842864990234375f) - (x_80 * 0.85373473167419433594f));
}

fn taylorInvSqrt_f1_(r_1 : ptr<function, f32>) -> f32 {
  let x_86 : f32 = *(r_1);
  return (1.792842864990234375f - (0.85373473167419433594f * x_86));
}

const C = vec4<f32>(0.138196602463722229f, 0.138196602463722229f, 0.138196602463722229f, 0.138196602463722229f);
const F4 = 0.30901700258255004883f;
fn snoise_vf4_(v : vec4<f32>) -> f32 {
  var i2 : vec4<f32>;
  var i1 : vec4<f32>;
  var x1 : vec4<f32>;
  var x2 : vec4<f32>;
  var x3 : vec4<f32>;
  var x4 : vec4<f32>;
  var param_2 : vec4<f32>;
  var j0 : f32;
  var param_3 : f32;
  var param_4 : f32;
  var param_5 : f32;
  var param_6 : f32;
  var j1 : vec4<f32>;
  var param_7 : vec4<f32>;
  var param_8 : vec4<f32>;
  var param_9 : vec4<f32>;
  var param_10 : vec4<f32>;
  var ip_1 : vec4<f32>;
  var p0 : vec4<f32>;
  var param_11 : f32;
  var param_12 : vec4<f32>;
  var p1 : vec4<f32>;
  var param_13 : f32;
  var param_14 : vec4<f32>;
  var p2 : vec4<f32>;
  var param_15 : f32;
  var param_16 : vec4<f32>;
  var p3 : vec4<f32>;
  var param_17 : f32;
  var param_18 : vec4<f32>;
  var p4 : vec4<f32>;
  var param_19 : f32;
  var param_20 : vec4<f32>;
  var norm : vec4<f32>;
  var param_21 : vec4<f32>;
  var param_22 : f32;
  var m0 : vec3<f32>;
  var m1 : vec2<f32>;

  var i = floor((v + dot(v, vec4<f32>(F4))));
  var x0: vec4<f32> = ((v - i) + dot(i, C));

  var i0 : vec4<f32>;
  var isX: vec3<f32> = step(x0.yzw, x0.xxx);
  var isYZ : vec3<f32> = step(x0.zww, x0.yyz);

  i0.x = isX.x + isX.y + isX.z;
  i0 = vec4<f32>(i0.x, (vec3<f32>(1.0f) - isX).xyz);
  i0.y += isYZ.x + isYZ.y;

  let temp1 : vec2<f32> = vec2<f32>(i0.zw) + vec2<f32>(1.0f) - vec2<f32>(isYZ.xy);
  i0 = vec4<f32>(i0.xy, temp1.xy);

  i0.z += isYZ.z;
  i0.w += 1.0f - isYZ.z;

  var i3 = clamp(i0, vec4<f32>(0.0f), vec4<f32>(1.0f));

  let x_233 : vec4<f32> = i0;
  i2 = clamp((x_233 - vec4<f32>(1.0f, 1.0f, 1.0f, 1.0f)), vec4<f32>(0.0f, 0.0f, 0.0f, 0.0f), vec4<f32>(1.0f, 1.0f, 1.0f, 1.0f));
  let x_240 : vec4<f32> = i0;
  i1 = clamp((x_240 - vec4<f32>(2.0f, 2.0f, 2.0f, 2.0f)), vec4<f32>(0.0f, 0.0f, 0.0f, 0.0f), vec4<f32>(1.0f, 1.0f, 1.0f, 1.0f));
  let x_247 : vec4<f32> = x0;
  let x_248 : vec4<f32> = i1;
  x1 = ((x_247 - x_248) + vec4<f32>(0.138196602463722229f, 0.138196602463722229f, 0.138196602463722229f, 0.138196602463722229f));
  let x_252 : vec4<f32> = x0;
  let x_253 : vec4<f32> = i2;
  x2 = ((x_252 - x_253) + vec4<f32>(0.27639320492744445801f, 0.27639320492744445801f, 0.27639320492744445801f, 0.27639320492744445801f));
  let x_259 : vec4<f32> = x0;
  let x_260 : vec4<f32> = i3;
  x3 = ((x_259 - x_260) + vec4<f32>(0.41458979249000549316f, 0.41458979249000549316f, 0.41458979249000549316f, 0.41458979249000549316f));
  let x_266 : vec4<f32> = x0;
  x4 = (x_266 + vec4<f32>(-0.44721359014511108398f, -0.44721359014511108398f, -0.44721359014511108398f, -0.44721359014511108398f));
  let x_271 : vec4<f32> = i;
  param_2 = x_271;
  let x_272 : vec4<f32> = mod289(param_2);
  i = x_272;
  let x_276 : f32 = i.w;
  param_3 = x_276;
  let x_277 : f32 = permute_f1_(&(param_3));
  let x_279 : f32 = i.z;
  param_4 = (x_277 + x_279);
  let x_282 : f32 = permute_f1_(&(param_4));
  let x_284 : f32 = i.y;
  param_5 = (x_282 + x_284);
  let x_287 : f32 = permute_f1_(&(param_5));
  let x_289 : f32 = i.x;
  param_6 = (x_287 + x_289);
  let x_292 : f32 = permute_f1_(&(param_6));
  j0 = x_292;
  let x_295 : f32 = i.w;
  let x_297 : f32 = i1.w;
  let x_299 : f32 = i2.w;
  let x_301 : f32 = i3.w;
  param_7 = (vec4<f32>(x_295, x_295, x_295, x_295) + vec4<f32>(x_297, x_299, x_301, 1.0f));
  let x_306 : vec4<f32> = permute_vf4_(&(param_7));
  let x_308 : f32 = i.z;
  let x_312 : f32 = i1.z;
  let x_314 : f32 = i2.z;
  let x_316 : f32 = i3.z;
  param_8 = ((x_306 + vec4<f32>(x_308, x_308, x_308, x_308)) + vec4<f32>(x_312, x_314, x_316, 1.0f));
  let x_320 : vec4<f32> = permute_vf4_(&(param_8));
  let x_322 : f32 = i.y;
  let x_326 : f32 = i1.y;
  let x_328 : f32 = i2.y;
  let x_330 : f32 = i3.y;
  param_9 = ((x_320 + vec4<f32>(x_322, x_322, x_322, x_322)) + vec4<f32>(x_326, x_328, x_330, 1.0f));
  let x_334 : vec4<f32> = permute_vf4_(&(param_9));
  let x_336 : f32 = i.x;
  let x_340 : f32 = i1.x;
  let x_342 : f32 = i2.x;
  let x_344 : f32 = i3.x;
  param_10 = ((x_334 + vec4<f32>(x_336, x_336, x_336, x_336)) + vec4<f32>(x_340, x_342, x_344, 1.0f));
  let x_348 : vec4<f32> = permute_vf4_(&(param_10));
  j1 = x_348;
  ip_1 = vec4<f32>(0.00340136047452688217f, 0.02040816284716129303f, 0.14285714924335479736f, 0.0f);
  let x_356 : f32 = j0;
  param_11 = x_356;
  let x_358 : vec4<f32> = ip_1;
  param_12 = x_358;
  let x_359 : vec4<f32> = grad4_f1_vf4_(&(param_11), &(param_12));
  p0 = x_359;
  let x_363 : f32 = j1.x;
  param_13 = x_363;
  let x_365 : vec4<f32> = ip_1;
  param_14 = x_365;
  let x_366 : vec4<f32> = grad4_f1_vf4_(&(param_13), &(param_14));
  p1 = x_366;
  let x_370 : f32 = j1.y;
  param_15 = x_370;
  let x_372 : vec4<f32> = ip_1;
  param_16 = x_372;
  let x_373 : vec4<f32> = grad4_f1_vf4_(&(param_15), &(param_16));
  p2 = x_373;
  let x_377 : f32 = j1.z;
  param_17 = x_377;
  let x_379 : vec4<f32> = ip_1;
  param_18 = x_379;
  let x_380 : vec4<f32> = grad4_f1_vf4_(&(param_17), &(param_18));
  p3 = x_380;
  let x_384 : f32 = j1.w;
  param_19 = x_384;
  let x_386 : vec4<f32> = ip_1;
  param_20 = x_386;
  let x_387 : vec4<f32> = grad4_f1_vf4_(&(param_19), &(param_20));
  p4 = x_387;
  let x_389 : vec4<f32> = p0;
  let x_390 : vec4<f32> = p0;
  let x_392 : vec4<f32> = p1;
  let x_393 : vec4<f32> = p1;
  let x_395 : vec4<f32> = p2;
  let x_396 : vec4<f32> = p2;
  let x_398 : vec4<f32> = p3;
  let x_399 : vec4<f32> = p3;
  param_21 = vec4<f32>(dot(x_389, x_390), dot(x_392, x_393), dot(x_395, x_396), dot(x_398, x_399));
  let x_403 : vec4<f32> = taylorInvSqrt_vf4_(&(param_21));
  norm = x_403;
  let x_405 : f32 = norm.x;
  let x_406 : vec4<f32> = p0;
  p0 = (x_406 * x_405);
  let x_409 : f32 = norm.y;
  let x_410 : vec4<f32> = p1;
  p1 = (x_410 * x_409);
  let x_413 : f32 = norm.z;
  let x_414 : vec4<f32> = p2;
  p2 = (x_414 * x_413);
  let x_417 : f32 = norm.w;
  let x_418 : vec4<f32> = p3;
  p3 = (x_418 * x_417);
  let x_420 : vec4<f32> = p4;
  let x_421 : vec4<f32> = p4;
  param_22 = dot(x_420, x_421);
  let x_424 : f32 = taylorInvSqrt_f1_(&(param_22));
  let x_425 : vec4<f32> = p4;
  p4 = (x_425 * x_424);
  let x_429 : vec4<f32> = x0;
  let x_430 : vec4<f32> = x0;
  let x_432 : vec4<f32> = x1;
  let x_433 : vec4<f32> = x1;
  let x_435 : vec4<f32> = x2;
  let x_436 : vec4<f32> = x2;
  m0 = max((vec3<f32>(0.60000002384185791016f, 0.60000002384185791016f, 0.60000002384185791016f) - vec3<f32>(dot(x_429, x_430), dot(x_432, x_433), dot(x_435, x_436))), vec3<f32>(0.0f, 0.0f, 0.0f));
  let x_445 : vec4<f32> = x3;
  let x_446 : vec4<f32> = x3;
  let x_448 : vec4<f32> = x4;
  let x_449 : vec4<f32> = x4;
  m1 = max((vec2<f32>(0.60000002384185791016f, 0.60000002384185791016f) - vec2<f32>(dot(x_445, x_446), dot(x_448, x_449))), vec2<f32>(0.0f, 0.0f));
  let x_456 : vec3<f32> = m0;
  let x_457 : vec3<f32> = m0;
  m0 = (x_456 * x_457);
  let x_459 : vec2<f32> = m1;
  let x_460 : vec2<f32> = m1;
  m1 = (x_459 * x_460);
  let x_463 : vec3<f32> = m0;
  let x_464 : vec3<f32> = m0;
  let x_466 : vec4<f32> = p0;
  let x_467 : vec4<f32> = x0;
  let x_469 : vec4<f32> = p1;
  let x_470 : vec4<f32> = x1;
  let x_472 : vec4<f32> = p2;
  let x_473 : vec4<f32> = x2;
  let x_477 : vec2<f32> = m1;
  let x_478 : vec2<f32> = m1;
  let x_480 : vec4<f32> = p3;
  let x_481 : vec4<f32> = x3;
  let x_483 : vec4<f32> = p4;
  let x_484 : vec4<f32> = x4;
  return (49.0f * (dot((x_463 * x_464), vec3<f32>(dot(x_466, x_467), dot(x_469, x_470), dot(x_472, x_473))) + dot((x_477 * x_478), vec2<f32>(dot(x_480, x_481), dot(x_483, x_484)))));
}