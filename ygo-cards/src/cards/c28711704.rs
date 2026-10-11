//! Toon Black Luster Soldier
use super::prelude::*;

pub const ID: u32 = 28711704;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_TOON_WORLD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must be properly summoned before reviving
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Special summon procedure (from hand)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(lua_fn(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(lua_fn(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Can attack directly
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e2, EFFECT_DIRECT_ATTACK);
    // e2:SetCondition(s.dircon)
    effect::set_condition(d, e2, Some(condition(dircon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Banish 1 card from the field
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e3, CATEGORY_REMOVE);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.rmcon)
    effect::set_condition(d, e3, Some(condition(rmcon)));
    // e3:SetCost(s.rmcost)
    effect::set_cost(d, e3, Some(cost_target(rmcost)));
    // e3:SetTarget(s.rmtg)
    effect::set_target(d, e3, Some(cost_target(rmtg)));
    // e3:SetOperation(s.rmop)
    effect::set_operation(d, e3, Some(operation(rmop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.spfilter(c)`
fn spfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsType(TYPE_TOON) and c:HasLevel() and c:IsLevelAbove(1) and c:IsReleasable()
    card::is_monster(d, c)
        && card::is_type(d, c, TYPE_TOON)
        && card::has_level(d, c)
        && card::is_level_above(d, c, 1)
        && card::is_releasable(d, c, None, None)
}
/// Sum of `Card.GetLevel` over a group
fn level_sum(d: &mut Duel, g: &CardSet) -> i64 {
    let mut s: i64 = 0;
    for tc in group::iter(d, g) {
        s += card::get_level(d, tc) as i64;
    }
    s
}
/// `s.rescon(sg,e,tp,mg)`: `(res, stop)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // if #sg>1 then
    let chk = aux::chkf_mmz(1)(d, sg, e, tp, mg, None).0;
    let ok = if sg.len() > 1 {
        // return aux.ChkfMMZ(1)(sg,e,tp,mg) and sg:GetSum(Card.GetLevel)>=8 and not sg:IsExists(Card.IsLevelAbove,1,nil,8)
        chk && level_sum(d, sg) >= 8 && !group::is_exists(d, sg, &|d, x| card::is_level_above(d, x, 8), 1, NO_EX)
    } else {
        // return aux.ChkfMMZ(1)(sg,e,tp,mg) and sg:GetSum(Card.GetLevel)>=8
        chk && level_sum(d, sg) >= 8
    };
    (ok, false)
}
/// `s.breakcon(sg,e,tp,mg)`
fn breakcon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:GetSum(Card.GetLevel)>=8
    (level_sum(d, sg) >= 8, false)
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    let c = match a.card(1) {
        Some(c) => c,
        None => return true,
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local rg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let rg = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x)), tp, LOCATION_HAND | LOCATION_MZONE, 0, h.into());
    // return aux.SelectUnselectGroup(rg,e,tp,1,99,s.rescon,0)
    matches!(
        aux::select_unselect_group(d, &rg, Some(e), tp, Some(1), Some(99), Some(&|d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, cc: Option<CardId>| rescon(d, sg, e, tp, mg, cc)), 0, tp, None, None, None, false),
        Err(true)
    )
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local rg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let rg = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x)), tp, LOCATION_HAND | LOCATION_MZONE, 0, h.into());
    // local mg=aux.SelectUnselectGroup(rg,e,tp,1,99,s.rescon,1,tp,HINTMSG_RELEASE,s.breakcon,s.breakcon,true)
    let mg = aux::select_unselect_group(
        d,
        &rg,
        Some(e),
        tp,
        Some(1),
        Some(99),
        Some(&|d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, cc: Option<CardId>| rescon(d, sg, e, tp, mg, cc)),
        1,
        tp,
        Some(HINTMSG_RELEASE as u64),
        Some(&|d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, cc: Option<CardId>| breakcon(d, sg, e, tp, mg, cc)),
        Some(&|d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, cc: Option<CardId>| breakcon(d, sg, e, tp, mg, cc)),
        true,
    )
    .unwrap_or_default();
    // if #mg>0 then
    if !mg.is_empty() {
        // mg:KeepAlive()
        // e:SetLabelObject(mg)
        effect::set_label_object(d, e, Value::Group(mg));
        // return true
        true
    } else {
        // return false
        false
    }
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if not g then return end
    let g = match effect::get_label_object(d, e).as_group().cloned() {
        Some(g) => g,
        None => return,
    };
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    // g:DeleteGroup()
}
/// `s.dircon(e)`
fn dircon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e);
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,15259703),tp,LOCATION_ONFIELD,0,1,nil)
    //     and not Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsType,TYPE_TOON),tp,0,LOCATION_MZONE,1,nil)
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_TOON_WORLD])), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
        && !duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_type(d, c, TYPE_TOON)), tp, 0, LOCATION_MZONE, 1, NO_EX)
}
/// `s.rmcon(e,tp,eg,ep,ev,re,r,rp)`
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,15259703),tp,LOCATION_ONFIELD,0,1,nil)
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_TOON_WORLD])), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
/// `s.rmcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:GetAttackAnnouncedCount()==0 end
    if a.chk == Some(0) {
        return card::get_attack_announced_count(d, c) == 0;
    }
    // Cannot attack this turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(3206)
    effect::set_description(d, e1, 3206);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_OATH+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // c:RegisterEffect(e1,true)
    card::register_effect(d, c, e1, true);
    true
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() and chkc:IsAbleToRemove() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_able_to_remove(d, chkc, None, None, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToRemove,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToRemove,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,0,LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, 0, LOCATION_ONFIELD as i32);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
            duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
}
