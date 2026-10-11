//! Destiny HERO - Dreadnought Master
use super::prelude::*;

pub const ID: u32 = 54641720;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[40591390],
    listed_series: &[SET_DESTINY_HERO],
    material_setcode: &[SET_DESTINY_HERO, SET_HERO],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 2 Level 5 or higher "Destiny HERO" monsters
    // Fusion.AddProcMixN(c,true,true,s.matfilter,2)
    fusion::add_proc_mix_n(d, c, true, true, vec![(fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| matfilter(d, c, fc, st, tp))), 2)]);
    // Must be either Fusion Summoned, or Special Summoned (from your Extra Deck) by Tributing 1 "Destiny HERO - Dreadmaster"
    // c:AddMustBeFusionSummoned()
    card::add_must_be_fusion_summoned(d, c);
    // local e0a=Effect.CreateEffect(c)
    let e0a = effect::create_effect(d, c);
    // e0a:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e0a, aux::stringid(ID, 0));
    // e0a:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e0a, EFFECT_TYPE_FIELD);
    // e0a:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0a, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0a:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e0a, EFFECT_SPSUMMON_PROC);
    // e0a:SetRange(LOCATION_EXTRA)
    effect::set_range(d, e0a, LOCATION_EXTRA);
    // e0a:SetCondition(s.selfspcon)
    effect::set_condition(d, e0a, Some(lua_fn(selfspcon)));
    // e0a:SetTarget(s.selfsptg)
    effect::set_target(d, e0a, Some(lua_fn(selfsptg)));
    // e0a:SetOperation(s.selfspop)
    effect::set_operation(d, e0a, Some(lua_fn(selfspop)));
    // e0a:SetValue(1)
    effect::set_value(d, e0a, 1);
    // c:RegisterEffect(e0a)
    card::register_effect(d, c, e0a, false);
    // You can only Special Summon "Destiny HERO - Dreadnought" once per turn this way, no matter which method you use
    // local e0b=Effect.CreateEffect(c)
    let e0b = effect::create_effect(d, c);
    // e0b:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e0b, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e0b:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e0b, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e0b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e0b, EVENT_SPSUMMON_SUCCESS);
    // e0b:SetCondition(s.regcon)
    effect::set_condition(d, e0b, Some(condition(regcon)));
    // e0b:SetOperation(s.regop)
    effect::set_operation(d, e0b, Some(operation(regop)));
    // c:RegisterEffect(e0b)
    card::register_effect(d, c, e0b, false);
    // If this card is Special Summoned: You can add 2 "Destiny HERO" monsters and/or cards that mention a "Destiny HERO" monster's card name from your Deck to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // This card's ATK becomes the total original ATK of all other "Destiny HERO" monsters in your field and GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_SET_ATTACK)
    effect::set_code(d, e2, EFFECT_SET_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(function(e,c) return Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsSetCard,SET_DESTINY_HERO),c:GetControler(),LOCATION_MZONE|LOCATION_GRAVE,0,c):GetSum(Card.GetBaseAttack) end)
    effect::set_value_fn(d, e2, lua_fn(atk_value));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.matfilter(c,fc,sumtype,sump)`
fn matfilter(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, sump: u8) -> bool {
    // return c:IsLevelAbove(5) and c:IsSetCard(SET_DESTINY_HERO,fc,sumtype,sump)
    card::is_level_above(d, c, 5) && card::is_set_card_ctx(d, c, &[SET_DESTINY_HERO as u16], Some(fc), sumtype, Some(sump))
}
/// `s.selfspcostfilter(c,tp,fc)`
fn selfspcostfilter(d: &mut Duel, c: CardId, tp: u8, fc: CardId) -> bool {
    // return c:IsSummonCode(fc,MATERIAL_FUSION,tp,40591390) and c:IsCanBeFusionMaterial(fc,MATERIAL_FUSION,tp)
    //     and Duel.GetLocationCountFromEx(tp,tp,c,fc)>0
    card::is_summon_code(d, c, Some(fc), MATERIAL_FUSION as u64, tp, &[40591390])
        && card::is_can_be_fusion_material(d, c, Some(fc), Some(MATERIAL_FUSION as u64), Some(tp))
        && duel::get_location_count_from_ex(d, tp, Some(tp), c.into(), duel::ExtraCard::Card(fc), None).0 > 0
}
/// `function(e,c)` of the Special Summon procedure (`s.selfspcon`)
fn selfspcon(d: &mut Duel, a: Args) -> bool {
    // if not c then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.CheckReleaseGroup(tp,s.selfspcostfilter,1,false,1,true,c,tp,nil,nil,nil,tp,c)
    let filter = move |d: &mut Duel, x: CardId| selfspcostfilter(d, x, tp, c);
    let ra = duel::ReleaseArgs {
        use_hand: Some(false),
        max: Some(1),
        check_field: true,
        to_check: Some(c),
        toplayer: Some(tp),
        ..Default::default()
    };
    duel::check_release_group(d, tp, Some(&filter), 1, ra, NO_EX, false)
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk,c)` of the Special Summon procedure (`s.selfsptg`)
fn selfsptg(d: &mut Duel, a: Args) -> bool {
    let (e, tp) = (a.e(0), a.p(1));
    // local c=... (the `c` argument, index 9)
    let c = a.c(9);
    // local g=Duel.SelectReleaseGroup(tp,s.selfspcostfilter,1,1,false,true,true,c,tp,nil,false,nil,tp,c)
    let filter = move |d: &mut Duel, x: CardId| selfspcostfilter(d, x, tp, c);
    let ra = duel::ReleaseArgs {
        use_hand: Some(false),
        cancelable: true,
        check_field: true,
        to_check: Some(c),
        toplayer: Some(tp),
        ..Default::default()
    };
    let g = duel::select_release_group(d, tp, Some(&filter), 1, 1, ra, NO_EX, false);
    // if g and #g>0 then
    if let Some(g) = g {
        if !g.is_empty() {
            // e:SetLabelObject(g)
            effect::set_label_object(d, e, Value::Group(g));
            // return true
            return true;
        }
    }
    // return false
    false
}
/// `function(e,tp,eg,ep,ev,re,r,rp,c)` of the Special Summon procedure (`s.selfspop`)
fn selfspop(d: &mut Duel, a: Args) {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e(0)).as_group().cloned();
    // if g and #g>0 then
    if let Some(g) = g {
        if !g.is_empty() {
            // Duel.Release(g,REASON_COST|REASON_MATERIAL)
            duel::release(d, g, REASON_COST | REASON_MATERIAL, None);
        }
    }
}
/// `s.regcon(e)`
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsFusionSummoned() or c:IsSummonType(SUMMON_TYPE_SPECIAL+1)
    card::is_fusion_summoned(d, c) || card::is_summon_type(d, c, &[SUMMON_TYPE_SPECIAL + 1])
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // You can only Special Summon "Destiny HERO - Dreadnought" once per turn this way, no matter which method you use
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c,sump,sumtype) return c:IsOriginalCodeRule(id) and (sumtype&SUMMON_TYPE_FUSION==SUMMON_TYPE_FUSION or sumtype&SUMMON_TYPE_SPECIAL+1==SUMMON_TYPE_SPECIAL+1) end)
    effect::set_target(d, e1, Some(lua_fn(regop_target)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `function(e,c,sump,sumtype) return c:IsOriginalCodeRule(id) and (...) end` (value of `s.regop`)
fn regop_target(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    let sumtype = a.int(3) as u64;
    card::is_original_code_rule(d, c, &[ID])
        && ((sumtype & SUMMON_TYPE_FUSION as u64) == SUMMON_TYPE_FUSION as u64
            || (sumtype & (SUMMON_TYPE_SPECIAL + 1) as u64) == (SUMMON_TYPE_SPECIAL + 1) as u64)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return ((c:IsSetCard(SET_DESTINY_HERO) and c:IsMonster()) or c:ListsCodeWithArchetype(SET_DESTINY_HERO)) and c:IsAbleToHand()
    ((card::is_set_card(d, c, &[SET_DESTINY_HERO as u16]) && card::is_monster(d, c)) || card::lists_code_with_archetype(d, c, &[SET_DESTINY_HERO]))
        && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,2,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 2, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 2, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,2,2,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 2, 2, false, NO_EX).nil();
    // if #g==2 then
    if g.len() == 2 {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
/// `e2:SetValue(function(e,c) ... GetSum(Card.GetBaseAttack) end)`
fn atk_value(d: &mut Duel, a: Args) -> i64 {
    // local c = the card (index 1)
    let c = a.c(1);
    // Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsSetCard,SET_DESTINY_HERO),c:GetControler(),LOCATION_MZONE|LOCATION_GRAVE,0,c)
    let p = card::get_controler(d, c);
    let f = aux::faceup_filter(|d, x| card::is_set_card(d, x, &[SET_DESTINY_HERO as u16]));
    let g = duel::get_matching_group(d, Some(&f), p, LOCATION_MZONE | LOCATION_GRAVE, 0, c.into());
    // :GetSum(Card.GetBaseAttack)
    group::get_sum(d, &g, &|d, x| card::get_base_attack(d, x) as i64)
}
