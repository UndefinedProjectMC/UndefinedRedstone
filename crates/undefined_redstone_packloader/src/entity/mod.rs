use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use bevy_ecs::bundle::Bundle;
use bevy_ecs::component::Component;
use bevy_ecs::system::Resource;
use bevy_ecs::world::{EntityWorldMut, World};
use serde::Deserialize;
use serde_json::{Map, Value};
use crate::components_export;
use crate::entity::component::*;
use crate::entity::behavior_component::*;

pub mod component;
pub mod behavior_component;

#[derive(Deserialize, Debug, Clone)]
pub struct MinecraftEntityDescription {
    pub identifier: String,
    pub spawn_category: Option<String>,
    #[serde(default)]
    pub is_spawnable: bool,
    #[serde(default)]
    pub is_summonable: bool,
}

#[derive(Component, Debug)]
pub struct MinecraftEntity {
    pub format_version: String,
    pub description: MinecraftEntityDescription,
}

#[derive(Deserialize, Clone)]
pub struct MinecraftEntityContent {
    #[serde(skip_deserializing)]
    pub format_version: String,
    pub description: MinecraftEntityDescription,
    pub components: Option<EntityComponents>,
    pub component_groups: Map<String, Value>,
}

impl MinecraftEntityContent {
    pub fn spawn<'a>(&'a self, world: &'a mut World) -> EntityWorldMut {
       let mut entity = world.spawn(MinecraftEntity {
          format_version: self.format_version.clone(),
          description: self.description.clone(),
       });
       if let Some(components) = self.components.clone() {
          components.spawn(&mut entity);
       }
       return entity;
    }

   pub fn insert<'a>(&'a self, entity: &'a mut EntityWorldMut) {
      entity.insert(MinecraftEntity {
         format_version: self.format_version.clone(),
         description: self.description.clone(),
      });
      if let Some(components) = self.components.clone() {
         components.spawn(entity);
      }
   }
}

impl Debug for MinecraftEntityContent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        //把component_groups的Value转成Vec<EntityComponents>
        let x = self.component_groups.iter().map(|(key, v)| {
            let x = serde_json::from_value::<EntityComponents>(v.clone()).unwrap();
            (key.clone(), x)
        }).collect::<HashMap<String, EntityComponents>>();
        f.debug_struct("MinecraftEntity")
            .field("format_version", &self.format_version)
            .field("description", &self.description)
            .field("component_groups", &x)
            .field("components", &self.components)
            .finish()
    }
}

#[derive(Resource, Clone)]
pub struct EntityContentManager {
    pub content: HashMap<String, Arc<MinecraftEntityContent>>,
}

impl EntityContentManager {
    pub fn new() -> Self {
        Self {
            content: HashMap::new(),
        }
    }

   pub fn from_map(map: HashMap<String, Arc<MinecraftEntityContent>>) -> Self {
      Self {
         content: map,
      }
   }

   pub fn extend(&mut self, map: HashMap<String, Arc<MinecraftEntityContent>>) {
      self.content.extend(map);
   }

   pub fn get(&self, name: &str) -> Option<Arc<MinecraftEntityContent>> {
      self.content.get(name).cloned()
   }

   pub fn insert(&mut self, name: String, content: Arc<MinecraftEntityContent>) {
      self.content.insert(name, content);
   }

   pub fn remove(&mut self, name: &str) -> Option<Arc<MinecraftEntityContent>> {
      self.content.remove(name)
   }

   pub fn len(&self) -> usize {
      self.content.len()
   }
}

components_export![
   EntityComponents,
   AddRider = "minecraft:addrider",
   AdmireItem = "minecraft:admire_item",
   Ageable = "minecraft:ageable",
   AmbientSoundInterval = "minecraft:ambient_sound_interval",
   AngerLevel = "minecraft:anger_level",
   Angry = "minecraft:angry",
   AnnotationBreakDoor = "minecraft:annotation_break_door",
   AnnotationOpenDoor = "minecraft:annotation_open_door",
   AreaAttack = "minecraft:area_attack",
   Attack = "minecraft:attack",
   AttackCooldown = "minecraft:attack_cooldown",
   AttackDamage = "minecraft:attack_damage",
   Balloonable = "minecraft:balloonable",
   Barter = "minecraft:barter",
   BlockClimber = "minecraft:block_climber",
   BlockSensor = "minecraft:block_sensor",
   BodyRotationBlocked = "minecraft:body_rotation_blocked",
   Boostable = "minecraft:boostable",
   Boss = "minecraft:boss",
   BreakBlocks = "minecraft:break_blocks",
   Breathable = "minecraft:breathable",
   Breedable = "minecraft:breedable",
   Bribeable = "minecraft:bribeable",
   Buoyant = "minecraft:buoyant",
   BurnsInDaylight = "minecraft:burns_in_daylight",
   CanClimb = "minecraft:can_climb",
   CanFly = "minecraft:can_fly",
   CanJoinRaid = "minecraft:can_join_raid",
   CanPowerJump = "minecraft:can_power_jump",
   CelebrateHunt = "minecraft:celebrate_hunt",
   Color = "minecraft:color",
   Color2 = "minecraft:color2",
   CombatRegeneration = "minecraft:combat_regeneration",
   ConditionalBandwidthOptimization = "minecraft:conditional_bandwidth_optimization",
   CustomHitTest = "minecraft:custom_hit_test",
   DamageOverTime = "minecraft:damage_over_time",
   DamageSensor = "minecraft:damage_sensor",
   Dash = "minecraft:dash",
   DefaultLookAngle = "minecraft:default_look_angle",
   Despawn = "minecraft:despawn",
   DryingOutTimer = "minecraft:drying_out_timer",
   Dweller = "minecraft:dweller",
   EconomyTradeTable = "minecraft:economy_trade_table",
   EntitySensor = "minecraft:entity_sensor",
   EnvironmentSensor = "minecraft:environment_sensor",
   EquipItem = "minecraft:equip_item",
   Equipment = "minecraft:equipment",
   Equippable = "minecraft:equippable",
   ExhaustionValues = "minecraft:exhaustion_values",
   ExperienceReward = "minecraft:experience_reward",
   Explode = "minecraft:explode",
   FireImmune = "minecraft:fire_immune",
   FloatsInLiquid = "minecraft:floats_in_liquid",
   Flocking = "minecraft:flocking",
   FlyingSpeed = "minecraft:flying_speed",
   FollowRange = "minecraft:follow_range",
   FrictionModifier = "minecraft:friction_modifier",
   GameEventMovementTracking = "minecraft:game_event_movement_tracking",
   Genetics = "minecraft:genetics",
   Giveable = "minecraft:giveable",
   GroundOffset = "minecraft:ground_offset",
   GroupSize = "minecraft:group_size",
   GrowsCrop = "minecraft:grows_crop",
   Healable = "minecraft:healable",
   Health = "minecraft:health",
   Heartbeat = "minecraft:heartbeat",
   Hide = "minecraft:hide",
   Home = "minecraft:home",
   HorseJumpStrength = "minecraft:horse.jump_strength",
   HurtOnCondition = "minecraft:hurt_on_condition",
   InputGroundControlled = "minecraft:input_ground_controlled",
   InsideBlockNotifier = "minecraft:inside_block_notifier",
   Insomnia = "minecraft:insomnia",
   InstantDespawn = "minecraft:instant_despawn",
   Interact = "minecraft:interact",
   Inventory = "minecraft:inventory",
   IsBaby = "minecraft:is_baby",
   IsCharged = "minecraft:is_charged",
   IsChested = "minecraft:is_chested",
   IsDyeable = "minecraft:is_dyeable",
   IsHiddenWhenInvisible = "minecraft:is_hidden_when_invisible",
   IsIgnited = "minecraft:is_ignited",
   IsIllagerCaptain = "minecraft:is_illager_captain",
   IsSaddled = "minecraft:is_saddled",
   IsShaking = "minecraft:is_shaking",
   IsSheared = "minecraft:is_sheared",
   IsStackable = "minecraft:is_stackable",
   IsStunned = "minecraft:is_stunned",
   IsTamed = "minecraft:is_tamed",
   ItemControllable = "minecraft:item_controllable",
   ItemHopper = "minecraft:item_hopper",
   JumpDynamic = "minecraft:jump.dynamic",
   JumpStatic = "minecraft:jump.static",
   KnockbackResistance = "minecraft:knockback_resistance",
   LavaMovement = "minecraft:lava_movement",
   Leashable = "minecraft:leashable",
   LookAt = "minecraft:lookat",
   Loot = "minecraft:loot",
   ManagedWanderingTrader = "minecraft:managed_wandering_trader",
   MarkVariant = "minecraft:mark_variant",
   MobEffect = "minecraft:mob_effect",
   MovementAmphibious = "minecraft:movement.amphibious",
   MovementBasic = "minecraft:movement.basic",
   MovementDolphin = "minecraft:movement.dolphin",
   MovementFly = "minecraft:movement.fly",
   MovementGeneric = "minecraft:movement.generic",
   MovementGlide = "minecraft:movement.glide",
   MovementHover = "minecraft:movement.hover",
   MovementJump = "minecraft:movement.jump",
   MovementSkip = "minecraft:movement.skip",
   MovementSoundDistanceOffset = "minecraft:movement_sound_distance_offset",
   MovementSway = "minecraft:movement.sway",
   Nameable = "minecraft:nameable",
   NavigationClimb = "minecraft:navigation.climb",
   NavigationFloat = "minecraft:navigation.float",
   NavigationFly = "minecraft:navigation.fly",
   NavigationGeneric = "minecraft:navigation.generic",
   NavigationHover = "minecraft:navigation.hover",
   NavigationSwim = "minecraft:navigation.swim",
   NavigationWalk = "minecraft:navigation.walk",
   OutOfControl = "minecraft:out_of_control",
   Peek = "minecraft:peek",
   Persistent = "minecraft:persistent",
   Physics = "minecraft:physics",
   PlayerExhaustion = "minecraft:player.exhaustion",
   PlayerExperience = "minecraft:player.experience",
   PlayerLevel = "minecraft:player.level",
   PlayerSaturation = "minecraft:player.saturation",
   PreferredPath = "minecraft:preferred_path",
   Projectile = "minecraft:projectile",
   PushThrough = "minecraft:push_through",
   Pushable = "minecraft:pushable",
   RaidTrigger = "minecraft:raid_trigger",
   RailMovement = "minecraft:rail_movement",
   RailSensor = "minecraft:rail_sensor",
   RavagerBlocked = "minecraft:ravager_blocked",
   Rideable = "minecraft:rideable",
   ScaffoldingClimber = "minecraft:scaffolding_climber",
   Scale = "minecraft:scale",
   ScaleByAge = "minecraft:scale_by_age",
   Scheduler = "minecraft:scheduler",
   Shareables = "minecraft:shareables",
   Shooter = "minecraft:shooter",
   Sittable = "minecraft:sittable",
   SkinId = "minecraft:skin_id",
   SoundVolume = "minecraft:sound_volume",
   SpawnEntity = "minecraft:spawn_entity",
   SpellEffects = "minecraft:spell_effects",
   Strength = "minecraft:strength",
   Tameable = "minecraft:tameable",
   Tamemount = "minecraft:tamemount",
   TargetNearbySensor = "minecraft:target_nearby_sensor",
   Teleport = "minecraft:teleport",
   TickWorld = "minecraft:tick_world",
   Timer = "minecraft:timer",
   TradeResupply = "minecraft:trade_resupply",
   TradeTable = "minecraft:trade_table",
   Trail = "minecraft:trail",
   Transformation = "minecraft:transformation",
   Trust = "minecraft:trust",
   Trusting = "minecraft:trusting",
   TypeFamily = "minecraft:type_family",
   UnderwaterMovement = "minecraft:underwater_movement",
   Variant = "minecraft:variant",
   WalkAnimationSpeed = "minecraft:walk_animation_speed",
   WantsJockey = "minecraft:wants_jockey",
   WaterMovement = "minecraft:water_movement",
   VariableMaxAutoStep = "minecraft:variable_max_auto_step",
   BehaviorAvoidBlock = "minecraft:behavior.avoid_block",
   BehaviorAvoidMobType = "minecraft:behavior.avoid_mob_type",
   BehaviorBarter = "minecraft:behavior.barter",
   BehaviorBeg = "minecraft:behavior.beg",
   BehaviorBreed = "minecraft:behavior.breed",
   BehaviorCelebrate = "minecraft:behavior.celebrate",
   BehaviorCelebrateSurvive = "minecraft:behavior.celebrate_survive",
   BehaviorChargeAttack = "minecraft:behavior.charge_attack",
   BehaviorChargeHeldItem = "minecraft:behavior.charge_held_item",
   BehaviorCircleAroundAnchor = "minecraft:behavior.circle_around_anchor",
   BehaviorControlledByPlayer = "minecraft:behavior.controlled_by_player",
   BehaviorCroak = "minecraft:behavior.croak",
   BehaviorDefendTrustedTarget = "minecraft:behavior.defend_trusted_target",
   BehaviorDefendVillageTarget = "minecraft:behavior.defend_village_target",
   BehaviorDelayedAttack = "minecraft:behavior.delayed_attack",
   BehaviorDig = "minecraft:behavior.dig",
   BehaviorDrinkMilk = "minecraft:behavior.drink_milk",
   BehaviorDragonChargePlayer = "minecraft:behavior.dragonchargeplayer",
   BehaviorDragondeath = "minecraft:behavior.dragondeath",
   BehaviorDragonflaming = "minecraft:behavior.dragonflaming",
   BehaviorDragonHoldingPattern = "minecraft:behavior.dragonholdingpattern",
   BehaviorDragonLanding = "minecraft:behavior.dragonlanding",
   BehaviorDragonScanning = "minecraft:behavior.dragonscanning",
   BehaviorDragonStrafePlayer = "minecraft:behavior.dragonstrafeplayer",
   BehaviorDragonTakeoff = "minecraft:behavior.dragontakeoff",
   BehaviorDrinkPotion = "minecraft:behavior.drink_potion",
   BehaviorDropItemFor = "minecraft:behavior.drop_item_for",
   BehaviorEatBlock = "minecraft:behavior.eat_block",
   BehaviorEatCarriedItem = "minecraft:behavior.eat_carried_item",
   BehaviorEatMob = "minecraft:behavior.eat_mob",
   BehaviorEmerge = "minecraft:behavior.emerge",
   BehaviorEndermanLeaveBlock = "minecraft:behavior.enderman_leave_block",
   BehaviorEndermanTakeBlock = "minecraft:behavior.enderman_take_block",
   BehaviorEquipItem = "minecraft:behavior.equip_item",
   BehaviorExploreOutskirts = "minecraft:behavior.explore_outskirts",
   BehaviorFertilizeFarmBlock = "minecraft:behavior.fertilize_farm_block",
   BehaviorFindCover = "minecraft:behavior.find_cover",
   BehaviorFindMount = "minecraft:behavior.find_mount",
   BehaviorFindUnderwaterTreasure = "minecraft:behavior.find_underwater_treasure",
   BehaviorFleeSun = "minecraft:behavior.flee_sun",
   BehaviorFloat = "minecraft:behavior.float",
   BehaviorFloatWander = "minecraft:behavior.float_wander",
   BehaviorFollowCaravan = "minecraft:behavior.follow_caravan",
   BehaviorFollowMob = "minecraft:behavior.follow_mob",
   BehaviorFollowOwner = "minecraft:behavior.follow_owner",
   BehaviorFollowParent = "minecraft:behavior.follow_parent",
   BehaviorFollowTargetCaptain = "minecraft:behavior.follow_target_captain",
   BehaviorGoAndGiveItemsToNoteblock = "minecraft:behavior.go_and_give_items_to_noteblock",
   BehaviorGoAndGiveItemsToOwner = "minecraft:behavior.go_and_give_items_to_owner",
   BehaviorGoHome = "minecraft:behavior.go_home",
   BehaviorGuardianAttack = "minecraft:behavior.guardian_attack",
   BehaviorHarvestFarmBlock = "minecraft:behavior.harvest_farm_block",
   BehaviorHide = "minecraft:behavior.hide",
   BehaviorHoldGround = "minecraft:behavior.hold_ground",
   BehaviorHurtByTarget = "minecraft:behavior.hurt_by_target",
   BehaviorInspectBookshelf = "minecraft:behavior.inspect_bookshelf",
   BehaviorInvestigateSuspiciousLocation = "minecraft:behavior.investigate_suspicious_location",
   BehaviorJumpToBlock = "minecraft:behavior.jump_to_block",
   BehaviorKnockbackRoar = "minecraft:behavior.knockback_roar",
   BehaviorLayDown = "minecraft:behavior.lay_down",
   BehaviorLayEgg = "minecraft:behavior.lay_egg",
   BehaviorLeapAtTarget = "minecraft:behavior.leap_at_target",
   BehaviorLookAtEntity = "minecraft:behavior.look_at_entity",
   BehaviorLookAtPlayer = "minecraft:behavior.look_at_player",
   BehaviorLookAtTarget = "minecraft:behavior.look_at_target",
   BehaviorLookAtTradingPlayer = "minecraft:behavior.look_at_trading_player",
   BehaviorMakeLove = "minecraft:behavior.make_love",
   BehaviorMeleeAttack = "minecraft:behavior.melee_attack",
   BehaviorMeleeBoxAttack = "minecraft:behavior.melee_box_attack",
   BehaviorMingle = "minecraft:behavior.mingle",
   BehaviorMountPathing = "minecraft:behavior.mount_pathing",
   BehaviorMoveIndoors = "minecraft:behavior.move_indoors",
   BehaviorMoveOutdoors = "minecraft:behavior.move_outdoors",
   BehaviorMoveThroughVillage = "minecraft:behavior.move_through_village",
   BehaviorMoveToBlock = "minecraft:behavior.move_to_block",
   BehaviorMoveToLand = "minecraft:behavior.move_to_land",
   BehaviorMoveToLiquid = "minecraft:behavior.move_to_liquid",
   BehaviorMoveToRandomBlock = "minecraft:behavior.move_to_random_block",
   BehaviorMoveToVillage = "minecraft:behavior.move_to_village",
   BehaviorMoveToWater = "minecraft:behavior.move_to_water",
   BehaviorMoveTowardsDwellingRestriction = "minecraft:behavior.move_towards_dwelling_restriction",
   BehaviorMoveTowardsHomeRestriction = "minecraft:behavior.move_towards_home_restriction",
   BehaviorMoveTowardsTarget = "minecraft:behavior.move_towards_target",
   BehaviorNap = "minecraft:behavior.nap",
   BehaviorNearestAttackableTarget = "minecraft:behavior.nearest_attackable_target",
   BehaviorNearestPrioritizedAttackableTarget = "minecraft:behavior.nearest_prioritized_attackable_target",
   BehaviorOcelotSitOnBlock = "minecraft:behavior.ocelot_sit_on_block",
   BehaviorOcelotAttack = "minecraft:behavior.ocelotattack",
   BehaviorOfferFlower = "minecraft:behavior.offer_flower",
   BehaviorOpenDoor = "minecraft:behavior.open_door",
   BehaviorOwnerHurtByTarget = "minecraft:behavior.owner_hurt_by_target",
   BehaviorOwnerHurtTarget = "minecraft:behavior.owner_hurt_target",
   BehaviorPanic = "minecraft:behavior.panic",
   BehaviorPeek = "minecraft:behavior.peek",
   BehaviorPetSleepWithOwner = "minecraft:behavior.pet_sleep_with_owner",
   BehaviorPickupItems = "minecraft:behavior.pickup_items",
   BehaviorPlay = "minecraft:behavior.play",
   BehaviorPlayDead = "minecraft:behavior.play_dead",
   BehaviorPlayerRideTamed = "minecraft:behavior.player_ride_tamed",
   BehaviorRaidGarden = "minecraft:behavior.raid_garden",
   BehaviorRamAttack = "minecraft:behavior.ram_attack",
   BehaviorRandomBreach = "minecraft:behavior.random_breach",
   BehaviorRandomFly = "minecraft:behavior.random_fly",
   BehaviorRandomHover = "minecraft:behavior.random_hover",
   BehaviorRandomLookAroundAndSit = "minecraft:behavior.random_look_around_and_sit",
   BehaviorRandomLookAround = "minecraft:behavior.random_look_around",
   BehaviorRandomSearchAndDig = "minecraft:behavior.random_search_and_dig",
   BehaviorRandomSitting = "minecraft:behavior.random_sitting",
   BehaviorRandomStroll = "minecraft:behavior.random_stroll",
   BehaviorRandomSwim = "minecraft:behavior.random_swim",
   BehaviorReceiveLove = "minecraft:behavior.receive_love",
   BehaviorRestrictOpenDoor = "minecraft:behavior.restrict_open_door",
   BehaviorRangedAttack = "minecraft:behavior.ranged_attack",
   BehaviorRestrictSun = "minecraft:behavior.restrict_sun",
   BehaviorRiseToLiquidLevel = "minecraft:behavior.rise_to_liquid_level",
   BehaviorRoar = "minecraft:behavior.roar",
   BehaviorRoll = "minecraft:behavior.roll",
   BehaviorRunAroundLikeCrazy = "minecraft:behavior.run_around_like_crazy",
   BehaviorScared = "minecraft:behavior.scared",
   BehaviorSendEvent = "minecraft:behavior.send_event",
   BehaviorShareItems = "minecraft:behavior.share_items",
   BehaviorSilverfishMergeWithStone = "minecraft:behavior.silverfish_merge_with_stone",
   BehaviorSilverfishWakeUpFriends = "minecraft:behavior.silverfish_wake_up_friends",
   BehaviorSkeletonHorseTrap = "minecraft:behavior.skeleton_horse_trap",
   BehaviorSleep = "minecraft:behavior.sleep",
   BehaviorSlimeAttack = "minecraft:behavior.slime_attack",
   BehaviorSlimeFloat = "minecraft:behavior.slime_float",
   BehaviorSlimeKeepOnJumping = "minecraft:behavior.slime_keep_on_jumping",
   BehaviorSlimeRandomDirection = "minecraft:behavior.slime_random_direction",
   BehaviorSnacking = "minecraft:behavior.snacking",
   BehaviorSneeze = "minecraft:behavior.sneeze",
   BehaviorSniff = "minecraft:behavior.sniff",
   BehaviorSonicBoom = "minecraft:behavior.sonic_boom",
   BehaviorSquidDive = "minecraft:behavior.squid_dive",
   BehaviorSquidFlee = "minecraft:behavior.squid_flee",
   BehaviorSquidIdle = "minecraft:behavior.squid_idle",
   BehaviorSquidMoveAwayFromGround = "minecraft:behavior.squid_move_away_from_ground",
   BehaviorSquidOutOfWater = "minecraft:behavior.squid_out_of_water",
   BehaviorStalkAndPounceOnTarget = "minecraft:behavior.stalk_and_pounce_on_target",
   BehaviorStayNearNoteblock = "minecraft:behavior.stay_near_noteblock",
   BehaviorStayWhileSitting = "minecraft:behavior.stay_while_sitting",
   BehaviorStompAttack = "minecraft:behavior.stomp_attack",
   BehaviorStompTurtleEgg = "minecraft:behavior.stomp_turtle_egg",
   BehaviorStrollTowardsVillage = "minecraft:behavior.stroll_towards_village",
   BehaviorSummonEntity = "minecraft:behavior.summon_entity",
   BehaviorSwell = "minecraft:behavior.swell",
   BehaviorSwimIdle = "minecraft:behavior.swim_idle",
   BehaviorSwimWander = "minecraft:behavior.swim_wander",
   BehaviorSwimWithEntity = "minecraft:behavior.swim_with_entity",
   BehaviorSwoopAttack = "minecraft:behavior.swoop_attack",
   BehaviorTakeFlower = "minecraft:behavior.take_flower",
   BehaviorTargetWhenPushed = "minecraft:behavior.target_when_pushed",
   BehaviorTempt = "minecraft:behavior.tempt",
   BehaviorTimerFlag1 = "minecraft:behavior.timer_flag_1",
   BehaviorTimerFlag2 = "minecraft:behavior.timer_flag_2",
   BehaviorTimerFlag3 = "minecraft:behavior.timer_flag_3",
   BehaviorTradeInterest = "minecraft:behavior.trade_interest",
   BehaviorTradeWithPlayer = "minecraft:behavior.trade_with_player",
   BehaviorVexCopyOwnerTarget = "minecraft:behavior.vex_copy_owner_target",
   BehaviorVexRandomMove = "minecraft:behavior.vex_random_move",
   BehaviorWitherRandomAttackPosGoal = "minecraft:behavior.wither_random_attack_pos_goal",
   BehaviorWitherTargetHighestDamage = "minecraft:behavior.wither_target_highest_damage",
   BehaviorWork = "minecraft:behavior.work",
   BehaviorWorkComposter = "minecraft:behavior.work_composter",
];