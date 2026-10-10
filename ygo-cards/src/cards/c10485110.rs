//! Ocean Dragon Lord - Neo-Daedalus
use super::prelude::*;

pub const ID: u32 = 10485110;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[37721209, CARD_UMI],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // c:AddMustBeSpecialSummoned()
    card::add_must_be_special_summoned(d, c);
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e0, aux::stringid(ID, 0));
    // e0:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e0, EFFECT_SPSUMMON_PROC);
    // e0:SetRange(LOCATION_HAND)
    effect::set_range(d, e0, LOCATION_HAND);
    // e0:SetCondition(function(e,c) ... end)
    effect::set_condition(d, e0, Some(lua_fn(spcon)));
    // e0:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk,c) ... end)
    effect::set_target(d, e0, Some(lua_fn(sptg)));
    // e0:SetOperation(function(e,tp,eg,ep,ev,re,r,rp,c) ... end)
    effect::set_operation(d, e0, Some(lua_fn(spop)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // You can send 1 face-up "Umi" you control to the GY; send all other cards on the field and all cards in both players' hands to the GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCost(s.tgcost)
    effect::set_cost(d, e1, Some(cost_target(tgcost)));
    // e1:SetTarget(s.tgtg)
    effect::set_target(d, e1, Some(cost_target(tgtg)));
    // e1:SetOperation(s.tgop)
    effect::set_operation(d, e1, Some(operation(tgop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tgcostfilter(c,hc)`
fn tgcostfilter(d: &mut Duel, c: CardId, hc: CardId) -> bool {
    // return c:IsCode(CARD_UMI) and c:IsFaceup() and c:IsAbleToGraveAsCost()
    //     and Duel.IsExistingMatchingCard(Card.IsAbleToGrave,0,LOCATION_HAND|LOCATION_ONFIELD,LOCATION_HAND|LOCATION_ONFIELD,1,Group.FromCards(c,hc))
    if !(card::is_code(d, c, &[CARD_UMI]) && card::is_faceup(d, c) && card::is_able_to_grave_as_cost(d, c)) {
        return false;
    }
    // Group.FromCards(c,hc)
    let mut ex = CardSet::from_card(c);
    group::add_card(&mut ex, &CardSet::from_card(hc));
    duel::is_existing_matching_card(d, Some(&|d, x| card::is_able_to_grave(d, x, None)), 0, LOCATION_HAND | LOCATION_ONFIELD, LOCATION_HAND | LOCATION_ONFIELD, 1, Exclude::Group(&ex))
}
/// `s.tgcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tgcostfilter,tp,LOCATION_ONFIELD,0,1,nil,c) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| tgcostfilter(d, x, c)), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.tgcostfilter,tp,LOCATION_ONFIELD,0,1,1,nil,c)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| tgcostfilter(d, x, c)), a.tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToGrave,tp,LOCATION_HAND|LOCATION_ONFIELD,LOCATION_HAND|LOCATION_ONFIELD,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&|d, x| card::is_able_to_grave(d, x, None)), a.tp, LOCATION_HAND | LOCATION_ONFIELD, LOCATION_HAND | LOCATION_ONFIELD, 1, Exclude::Card(h));
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,PLAYER_ALL,LOCATION_HAND|LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, PLAYER_ALL as u8, (LOCATION_HAND | LOCATION_ONFIELD) as i32);
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local exc=c:IsRelateToEffect(e) and c or nil
    let exc = if card::is_relate_to_effect(d, c, a.e) { Some(c) } else { None };
    // local g=Duel.GetMatchingGroup(Card.IsAbleToGrave,tp,LOCATION_HAND|LOCATION_ONFIELD,LOCATION_HAND|LOCATION_ONFIELD,exc)
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_able_to_grave(d, x, None)), a.tp, LOCATION_HAND | LOCATION_ONFIELD, LOCATION_HAND | LOCATION_ONFIELD, exc.into());
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoGrave(g,REASON_EFFECT)
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
/// `function(e,c)` of `e0:SetCondition`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.CheckReleaseGroup(tp,Card.IsCode,1,false,1,true,c,tp,nil,false,nil,37721209)
    duel::check_release_group(
        d,
        tp,
        Some(&|d, x| card::is_code(d, x, &[37721209])),
        1,
        duel::ReleaseArgs { use_hand: Some(false), max: Some(1), check_field: true, to_check: Some(c), toplayer: Some(tp), ..Default::default() },
        NO_EX,
        false,
    )
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk,c)` of `e0:SetTarget`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let c = a.c(9);
    // local g=Duel.SelectReleaseGroup(tp,Card.IsCode,1,1,false,true,true,c,nil,nil,false,nil,37721209)
    let g = duel::select_release_group(
        d,
        tp,
        Some(&|d, x| card::is_code(d, x, &[37721209])),
        1,
        1,
        duel::ReleaseArgs { use_hand: Some(false), cancelable: true, check_field: true, to_check: Some(c), ..Default::default() },
        NO_EX,
        false,
    );
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
/// `function(e,tp,eg,ep,ev,re,r,rp,c)` of `e0:SetOperation`
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if g and #g>0 then
    if let Value::Group(g) = effect::get_label_object(d, e) {
        if !g.is_empty() {
            // Duel.Release(g,REASON_COST)
            duel::release(d, g, REASON_COST, None);
        }
    }
}
