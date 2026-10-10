//! Flash Charge Dragon
use super::prelude::*;

pub const ID: u32 = 95372220;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // link summon
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_DRAGON),2)
    link::add_procedure(d, c, Some(mat_filter(|d, x, sc, st, tp| card::is_race_ctx(d, x, RACE_DRAGON, sc, st, Some(tp)))), 2, None, None, None, None);
    // force mzone
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_FORCE_MZONE)
    effect::set_code(d, e1, EFFECT_FORCE_MZONE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetValue(function(e)return ~e:GetHandler():GetLinkedZone() end)
    effect::set_value_fn(d, e1, lua_fn(forced_zone));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_REMOVE+CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_REMOVE + CATEGORY_DAMAGE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,0,EFFECT_COUNT_CODE_SINGLE)
    effect::set_count_limit(d, e2, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // negate
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_NEGATE)
    effect::set_category(d, e4, CATEGORY_NEGATE);
    // e4:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    // e4:SetCode(EVENT_CHAINING)
    effect::set_code(d, e4, EVENT_CHAINING);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e4, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e4:SetCondition(s.negcon)
    effect::set_condition(d, e4, Some(condition(negcon)));
    // e4:SetCost(s.negcost)
    effect::set_cost(d, e4, Some(cost_target(negcost)));
    // e4:SetTarget(s.negtg)
    effect::set_target(d, e4, Some(cost_target(negtg)));
    // e4:SetOperation(s.negop)
    effect::set_operation(d, e4, Some(operation(negop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `function(e)return ~e:GetHandler():GetLinkedZone() end` (the value of `e1`)
fn forced_zone(d: &mut Duel, a: Args) -> i64 {
    let h = effect::get_handler(d, a.e(0)).nil();
    !(card::get_linked_zone(d, h, None) as i64)
}
/// `s.cfilter(c,ec)`
fn cfilter(d: &mut Duel, c: CardId, ec: CardId) -> bool {
    // if c:IsLocation(LOCATION_MZONE) then
    if card::is_location(d, c, LOCATION_MZONE) {
        // return ec:GetLinkedGroup():IsContains(c)
        let g = card::get_linked_group(d, ec);
        group::is_contains(&g, c)
    } else {
        // return (ec:GetLinkedZone(c:GetPreviousControler())&(0x1<<c:GetPreviousSequence()))~=0
        let prev_controler = card::get_previous_controler(d, c);
        let zone = card::get_linked_zone(d, ec, Some(prev_controler));
        let prev_sequence = card::get_previous_sequence(d, c);
        (zone as u64) & (1u64 << prev_sequence) != 0
    }
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,e:GetHandler())
    let eg = a.eg.clone().nil();
    let h = effect::get_handler(d, a.e).nil();
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, h), 1, NO_EX)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=e:GetHandler():GetLinkedGroup()
    let h = effect::get_handler(d, a.e).nil();
    let g = card::get_linked_group(d, h);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,500)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 500);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=e:GetHandler():GetLinkedGroup()
    let h = effect::get_handler(d, a.e).nil();
    let g = card::get_linked_group(d, h);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local tg=g:Select(tp,1,1,nil)
    let tg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.HintSelection(tg)
        duel::hint_selection(d, tg.clone(), true);
        // if Duel.Destroy(tg,REASON_EFFECT)>0 then
        if duel::destroy(d, tg, REASON_EFFECT, None, None) > 0 {
            // Duel.Damage(1-tp,500,REASON_EFFECT)
            duel::damage(d, 1 - a.tp, 500, REASON_EFFECT, false, None);
        }
    }
}
/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsStatus(STATUS_BATTLE_DESTROYED) then return false end
    if card::is_status(d, c, STATUS_BATTLE_DESTROYED) {
        return false;
    }
    // if not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return end
    let re = a.re.nil();
    if !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    // local tg=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let tg = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned();
    // return tg and tg:IsContains(c) and Duel.IsChainNegatable(ev)
    match tg {
        None => false,
        Some(tg) => group::is_contains(&tg, c) && duel::is_chain_negatable(d, a.ev as u8),
    }
}
/// `s.costfilter(c)`
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return not c:IsStatus(STATUS_BATTLE_DESTROYED)
    !card::is_status(d, c, STATUS_BATTLE_DESTROYED)
}
/// `s.negcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckReleaseGroup(tp,s.costfilter,1,nil) end
    if a.chk == Some(0) {
        return duel::check_release_group(d, a.tp, Some(&costfilter), 1, duel::ReleaseArgs::default(), NO_EX, false);
    }
    // local g=Duel.SelectReleaseGroup(tp,s.costfilter,1,1,nil)
    let g = duel::select_release_group(d, a.tp, Some(&costfilter), 1, 1, duel::ReleaseArgs::default(), NO_EX, false).nil();
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    true
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) && effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE) {
        let re = a.re.nil();
        let re_handler = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, re_handler, re) {
            // Duel.SendtoGrave(eg,REASON_EFFECT)
            duel::send_to_grave(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
}
