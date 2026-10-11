//! The Great Gallant Bandit
//! 天下独歩の大義賊
use super::prelude::*;

pub const ID: u32 = 24203749;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If you control no monsters, you can Tribute Summon this card face-up by Tributing monsters from your hand and/or your opponent's field
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e0, aux::stringid(ID, 0));
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE | EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_SUMMON_PROC)
    effect::set_code(d, e0, EFFECT_SUMMON_PROC);
    // e0:SetCondition(function(e,c,minc,zone,relzone,exeff)
    effect::set_condition(d, e0, Some(lua_fn(summon_condition)));
    // e0:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk,c,minc,zone,relzone,exeff)
    effect::set_target(d, e0, Some(lua_fn(summon_target)));
    // e0:SetOperation(function(e,tp,eg,ep,ev,re,r,rp,c,minc,zone,relzone,exeff)
    effect::set_operation(d, e0, Some(lua_fn(summon_operation)));
    // e0:SetValue(SUMMON_TYPE_TRIBUTE)
    effect::set_value(d, e0, SUMMON_TYPE_TRIBUTE as i64);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // When your opponent activates a card effect in the hand, GY, or banishment, while you control this Normal Summoned/Set card (Quick Effect): You can negate that effect, and if you do, destroy that card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_DISABLE+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DISABLE | CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e,tp,eg,ep,ev,re,r,rp,chk)
    effect::set_condition(d, e1, Some(condition(neg_condition)));
    // e1:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk)
    effect::set_target(d, e1, Some(cost_target(neg_target)));
    // e1:SetOperation(function(e,tp,eg,ep,ev,re,r,rp)
    effect::set_operation(d, e1, Some(operation(neg_operation)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // During the Battle Phase (Quick Effect): You can reveal your entire hand; change all monsters your opponent controls to Defense Position
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e2, aux::stringid(ID, 2));
    // e2:SetCategory(CATEGORY_POSITION)
    effect::set_category(d, e2, CATEGORY_POSITION);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function() return Duel.IsBattlePhase() end)
    effect::set_condition(d, e2, Some(condition(|d, _a| duel::is_battle_phase(d, None))));
    // e2:SetCost(function(e,tp,eg,ep,ev,re,r,rp,chk)
    effect::set_cost(d, e2, Some(cost_target(reveal_cost)));
    // e2:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk)
    effect::set_target(d, e2, Some(cost_target(pos_target)));
    // e2:SetOperation(function(e,tp,eg,ep,ev,re,r,rp)
    effect::set_operation(d, e2, Some(operation(pos_operation)));
    // e2:SetHintTiming(0,TIMING_BATTLE_START|TIMING_BATTLE_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_BATTLE_START | TIMING_BATTLE_END | TIMINGS_CHECK_MONSTER));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `Card.IsReleasable` filter of the Tribute Summon: `Duel.GetMatchingGroup(Card.IsReleasable,tp,LOCATION_HAND,LOCATION_MZONE,c,tp)`
/// (the extra `tp` argument is the reason of `IsReleasable`).
fn releasable_group(d: &mut Duel, tp: u8, c: CardId) -> CardSet {
    duel::get_matching_group(d, Some(&|d, cc| card::is_releasable(d, cc, Some(tp as u32), None)), tp, LOCATION_HAND, LOCATION_MZONE, c.into())
}
/// `function(e,c,minc,zone,relzone,exeff)`
fn summon_condition(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local min_tributes=c:GetTributeRequirement()
    let (min_tributes, _max) = card::get_tribute_requirement(d, c);
    // if min_tributes<=0 then return false end
    if min_tributes <= 0 {
        return false;
    }
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 or Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)>0 then return false end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 || duel::get_field_group_count(d, tp, LOCATION_MZONE, 0) > 0 {
        return false;
    }
    // local mg=Duel.GetMatchingGroup(Card.IsReleasable,tp,LOCATION_HAND,LOCATION_MZONE,c,tp)
    let mg = releasable_group(d, tp, c);
    // return #mg>0 and #mg>=min_tributes
    !mg.is_empty() && mg.len() as i32 >= min_tributes
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk,c,minc,zone,relzone,exeff)`
fn summon_target(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let c = a.c(9);
    // local min_tributes,max_tributes=c:GetTributeRequirement()
    let (min_tributes, max_tributes) = card::get_tribute_requirement(d, c);
    // local mg=Duel.GetMatchingGroup(Card.IsReleasable,tp,LOCATION_HAND,LOCATION_MZONE,c,tp)
    let mg = releasable_group(d, tp, c);
    // local sg=aux.SelectUnselectGroup(mg,e,tp,min_tributes,max_tributes,nil,1,tp,HINTMSG_RELEASE,nil,nil,true)
    let sg = aux::select_unselect_group(
        d,
        &mg,
        Some(e),
        tp,
        Some(min_tributes as usize),
        Some(max_tributes as usize),
        None,
        1,
        tp,
        Some(HINTMSG_RELEASE as u64),
        None,
        None,
        true,
    )
    .unwrap_or_default();
    // if sg and #sg>0 then
    if !sg.is_empty() {
        // e:SetLabelObject(sg)
        effect::set_label_object(d, e, Value::Group(sg));
        // return true
        return true;
    }
    // return false
    false
}
/// `function(e,tp,eg,ep,ev,re,r,rp,c,minc,zone,relzone,exeff)`
fn summon_operation(d: &mut Duel, a: Args) {
    let e = a.e(0);
    let c = a.c(8);
    // local sg=e:GetLabelObject()
    let sg = effect::get_label_object(d, e).as_group().cloned();
    // if sg and #sg>0 then
    if let Some(sg) = sg {
        if !sg.is_empty() {
            // c:SetMaterial(sg)
            card::set_material(d, c, sg.clone());
            // Duel.Release(sg,REASON_SUMMON|REASON_MATERIAL)
            duel::release(d, sg, REASON_SUMMON | REASON_MATERIAL, None);
        }
    }
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk)` of e1
fn neg_condition(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return ep==1-tp and e:GetHandler():IsNormalSummoned() and Chain.IsTriggeringLocation(ev,LOCATION_HAND|LOCATION_GRAVE|LOCATION_REMOVED)
    //     and Duel.IsChainDisablable(ev)
    if a.ep != 1 - tp {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_normal_summoned(d, h) {
        return false;
    }
    if !chain::is_triggering(d, a.ev as u8, chain::Prop::Location, &[(LOCATION_HAND | LOCATION_GRAVE | LOCATION_REMOVED) as i64]).truthy() {
        return false;
    }
    duel::is_chain_disablable(d, a.ev as u8)
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk)` of e1
fn neg_target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, tp, 0);
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // if rc:IsDestructable() and rc:IsRelateToEffect(re) then
    if card::is_destructable(d, rc, None) && card::is_relate_to_effect(d, rc, re) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, tp, 0);
    }
    true
}
/// `function(e,tp,eg,ep,ev,re,r,rp)` of e1
fn neg_operation(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateEffect(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_effect(d, a.ev as u8) {
        let re = a.re.nil();
        let rc = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rc, re) {
            // Duel.Destroy(eg,REASON_EFFECT)
            duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk)` cost of e2
fn reveal_cost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetFieldGroup(tp,LOCATION_HAND,0)
    let g = duel::get_field_group(d, tp, LOCATION_HAND, 0);
    // if chk==0 then return #g>0 and not g:IsExists(Card.IsPublic,1,nil) end
    if a.chk == Some(0) {
        return !g.is_empty() && !group::is_exists(d, &g, &|d, c| card::is_public(d, c), 1, NO_EX);
    }
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - tp, g);
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, tp);
    true
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk)` target of e2
fn pos_target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.AND(Card.IsAttackPos,Card.IsCanChangePosition),tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_attack_pos(d, c) && card::is_can_change_position(d, c)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,nil,1,1-tp,POS_FACEUP_DEFENSE)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, None, 1, 1 - tp, POS_FACEUP_DEFENSE as i32);
    true
}
/// `function(e,tp,eg,ep,ev,re,r,rp)` operation of e2
fn pos_operation(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(aux.AND(Card.IsAttackPos,Card.IsCanChangePosition),tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_attack_pos(d, c) && card::is_can_change_position(d, c)), tp, 0, LOCATION_MZONE, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.ChangePosition(g,POS_FACEUP_DEFENSE)
        duel::change_position(d, g, POS_FACEUP_DEFENSE as u8, None, None, None, false);
    }
}
