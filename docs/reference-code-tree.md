# MonCheri reference code tree

Status: draft. Reference format accepted for first design.

moncheri_reference_system
├─ purpose
│  ├─ give every trackable entity a stable reference
│  ├─ make identical boxes and containers distinguishable
│  ├─ support moving flats, storage units, rooms, zones, stacks, containers, and items
│  ├─ keep references readable on labels and in search results
│  └─ keep references independent from current physical location
│
├─ reference_format
│  ├─ pattern
│  │  └─ HH-TYP-0001
│  ├─ HH
│  │  └─ household
│  ├─ TYP
│  │  └─ three-letter entity type code
│  ├─ 0001
│  │  └─ four-digit running number inside that entity type
│  └─ examples
│     ├─ HH-SIT-0001
│     ├─ HH-ROM-0001
│     ├─ HH-ZON-0001
│     ├─ HH-STK-0001
│     ├─ HH-CON-0001
│     ├─ HH-ITM-0001
│     ├─ HH-CAT-0001
│     └─ HH-TAG-0001
│
├─ casing_rule
│  ├─ database_stores
│  │  └─ uppercase canonical reference
│  ├─ app_displays
│  │  └─ uppercase canonical reference
│  ├─ user_input
│  │  └─ case-insensitive
│  └─ normalisation
│     ├─ hh-con-0001 becomes HH-CON-0001
│     ├─ Hh-Con-0001 becomes HH-CON-0001
│     └─ HH-CON-0001 stays HH-CON-0001
│
├─ entity_type_codes
│  ├─ HH-SIT-0001
│  │  ├─ code: SIT
│  │  ├─ name: site
│  │  ├─ meaning: whole physical place
│  │  └─ examples
│  │     ├─ flat
│  │     ├─ house
│  │     ├─ storage_unit_1
│  │     ├─ storage_unit_2
│  │     └─ temporary_work_flat
│  │
│  ├─ HH-ROM-0001
│  │  ├─ code: ROM
│  │  ├─ name: room
│  │  ├─ meaning: room or main internal area inside a site
│  │  └─ examples
│  │     ├─ bedroom
│  │     ├─ living_room
│  │     ├─ hallway
│  │     ├─ bathroom
│  │     └─ storage_unit_main_area
│  │
│  ├─ HH-ZON-0001
│  │  ├─ code: ZON
│  │  ├─ name: zone
│  │  ├─ meaning: described area, corner, wall, floor area, or side of a room/site
│  │  └─ examples
│  │     ├─ bedroom_window_left_corner
│  │     ├─ bedroom_door_right_corner
│  │     ├─ living_room_wall_beside_radiator
│  │     ├─ storage_unit_back_left_floor_area
│  │     └─ hallway_near_front_door
│  │
│  ├─ HH-STK-0001
│  │  ├─ code: STK
│  │  ├─ name: stack
│  │  ├─ meaning: stack, pile, or grouped set of containers
│  │  └─ examples
│  │     ├─ yellow_dewalt_box_stack
│  │     ├─ clear_plastic_box_stack
│  │     ├─ floor_stack_near_wall
│  │     └─ moving_boxes_stack
│  │
│  ├─ HH-CON-0001
│  │  ├─ code: CON
│  │  ├─ name: container
│  │  ├─ meaning: movable thing that can contain items
│  │  └─ examples
│  │     ├─ dewalt_box_1
│  │     ├─ dewalt_box_2
│  │     ├─ green_thule_rucksack
│  │     ├─ folder
│  │     ├─ drawer_unit
│  │     └─ pouch
│  │
│  ├─ HH-ITM-0001
│  │  ├─ code: ITM
│  │  ├─ name: item
│  │  ├─ meaning: trackable household thing
│  │  └─ examples
│  │     ├─ tweezers
│  │     ├─ black_socks
│  │     ├─ bike_charger
│  │     ├─ lipstick
│  │     └─ documents
│  │
│  ├─ HH-CAT-0001
│  │  ├─ code: CAT
│  │  ├─ name: category
│  │  ├─ meaning: structured classification
│  │  └─ examples
│  │     ├─ clothing
│  │     ├─ tools
│  │     ├─ documents
│  │     ├─ cosmetics
│  │     ├─ electronics
│  │     └─ bike_parts
│  │
│  └─ HH-TAG-0001
│     ├─ code: TAG
│     ├─ name: tag
│     ├─ meaning: flexible label or search helper
│     └─ examples
│        ├─ travel
│        ├─ urgent
│        ├─ seasonal
│        ├─ work
│        └─ rarely_used
│
├─ graph_rule
│  ├─ reference_code
│  │  └─ stable identity
│  ├─ graph_relation
│  │  └─ current location or classification
│  └─ move_logic
│     ├─ item reference stays the same
│     ├─ container reference stays the same
│     ├─ site reference stays the same
│     └─ currently_at relation changes
│
├─ numbering_rule
│  ├─ each_type_has_own_sequence
│  ├─ next SIT after HH-SIT-0001 is HH-SIT-0002
│  ├─ next CON after HH-CON-0001 is HH-CON-0002
│  ├─ next ITM after HH-ITM-0001 is HH-ITM-0002
│  └─ numbers_are_not_reused_after_delete
│
├─ physical_label_rule
│  ├─ label_sites
│  │  └─ usually no physical label needed
│  ├─ label_rooms
│  │  └─ usually no physical label needed
│  ├─ label_zones
│  │  └─ useful if room/storage layout is complex
│  ├─ label_stacks
│  │  └─ useful
│  ├─ label_containers
│  │  └─ strongly useful
│  ├─ label_items
│  │  └─ optional; useful only for valuable, sensitive, duplicated, or easily confused items
│  ├─ label_categories
│  │  └─ no physical label needed
│  └─ label_tags
│     └─ no physical label needed
│
├─ database_lookup
│  ├─ reference_type_table
│  │  ├─ type_code
│  │  ├─ type_name
│  │  ├─ meaning
│  │  ├─ example_ref
│  │  └─ next_number
│  └─ purpose
│     ├─ app_can_display_reference_help
│     ├─ backend_can_generate_next_reference
│     └─ user_can_check_abbreviations_if_forgotten
│
└─ first_accepted_reference_set
   ├─ HH-SIT-0001
   ├─ HH-ROM-0001
   ├─ HH-ZON-0001
   ├─ HH-STK-0001
   ├─ HH-CON-0001
   ├─ HH-ITM-0001
   ├─ HH-CAT-0001
   └─ HH-TAG-0001
