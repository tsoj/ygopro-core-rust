//! Elemental HERO Cosmo Neos
use super::prelude::*;

pub const ID: u32 = 90050480;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_NEOS],
    material_setcode: &[SET_HERO, SET_ELEMENTAL_HERO, SET_NEOS, SET_NEO_SPACIAN],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMixN(c,false,false,CARD_NEOS,1,s.ffilter,3)
    fusion::add_proc_mix_n(
        d,
        c,
        false,
        false,
        vec![(fusion::Mat::Code(CARD_NEOS), 1), (fusion::Mat::F(fusion::fus_mat_full(|d, x, a| ffilter(d, x, a))), 3)],
    );
    // Fusion.AddContactProc(c,s.contactfil,s.contactop,s.splimit)
    fusion::add_contact_proc(
        d,
        c,
        fusion::Contact {
            material_filter: Some(std::sync::Arc::new(contactfil)),
            material_operation: Some(std::sync::Arc::new(contactop)),
            sumcon: Some(Some(lua_fn(splimit))),
            ..Default::default()
        },
    );
    // no activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(s.nacon)
    effect::set_condition(d, e1, Some(condition(nacon)));
    // e1:SetTarget(s.natg)
    effect::set_target(d, e1, Some(cost_target(natg)));
    // e1:SetOperation(s.naop)
    effect::set_operation(d, e1, Some(operation(naop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // neos return
    // aux.EnableNeosReturn(c,CATEGORY_DESTROY,s.desinfo,s.desop)
    aux::enable_neos_return(d, c, Some(CATEGORY_DESTROY), Some(operation(desinfo)), Some(operation(desop)), None);
}
// function s.ffilter(c,fc,sumtype,tp,sub,mg,sg)
fn ffilter(d: &mut Duel, c: CardId, a: &fusion::MatArgs) -> bool {
    // return c:IsSetCard(SET_NEO_SPACIAN,fc,sumtype,tp) and c:GetAttribute(fc,sumtype,tp)~=0
    //     and (not sg or not sg:IsExists(s.fusfilter,1,c,c:GetAttribute(fc,sumtype,tp),fc,sumtype,tp))
    if !card::is_set_card_ctx(d, c, &[SET_NEO_SPACIAN as u16], Some(a.fc), a.sumtype, Some(a.tp)) {
        return false;
    }
    let attr = card::get_attribute_ctx(d, c, Some(a.fc), a.sumtype, Some(a.tp));
    if attr == 0 {
        return false;
    }
    match a.sg {
        None => true,
        Some(sg) => !group::is_exists(d, sg, &|d, x| fusfilter(d, x, attr, a.fc, a.sumtype, a.tp), 1, c.into()),
    }
}
// function s.fusfilter(c,attr,fc,sumtype,tp)
fn fusfilter(d: &mut Duel, c: CardId, attr: u32, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsSetCard(SET_NEO_SPACIAN,fc,sumtype,tp) and c:IsAttribute(attr,fc,sumtype,tp) and not c:IsHasEffect(511002961)
    card::is_set_card_ctx(d, c, &[SET_NEO_SPACIAN as u16], Some(fc), sumtype, Some(tp))
        && card::is_attribute_ctx(d, c, attr, Some(fc), sumtype, Some(tp))
        && card::is_has_effect(d, c, 511002961, None).is_empty()
}
// function s.contactfil(tp)
fn contactfil(d: &mut Duel, tp: u8) -> CardSet {
    // return Duel.GetMatchingGroup(Card.IsAbleToDeckOrExtraAsCost,tp,LOCATION_ONFIELD,0,nil)
    duel::get_matching_group(d, Some(&|d, c| card::is_able_to_deck_or_extra_as_cost(d, c)), tp, LOCATION_ONFIELD, 0, NO_EX)
}
// function s.contactop(g,tp)
fn contactop(d: &mut Duel, g: &CardSet, tp: u8, _c: CardId) {
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - tp, g.clone());
    // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_COST|REASON_MATERIAL)
    duel::send_to_deck(d, g.clone(), None, SEQ_DECKSHUFFLE as i32, REASON_COST | REASON_MATERIAL, None);
}
// function s.splimit(e,se,sp,st)
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return not e:GetHandler():IsLocation(LOCATION_EXTRA)
    let h = effect::get_handler(d, a.e(0)).nil();
    !card::is_location(d, h, LOCATION_EXTRA)
}
// function s.nacon(e)
fn nacon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetSummonLocation()&LOCATION_EXTRA==LOCATION_EXTRA
    let h = effect::get_handler(d, a.e).nil();
    (card::get_summon_location(d, h) & LOCATION_EXTRA) == LOCATION_EXTRA
}
// function s.natg(e,tp,eg,ep,ev,re,r,rp,chk)
fn natg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetChainLimit(s.chainlm)
    duel::set_chain_limit(d, lua_fn(chainlm));
    true
}
// function s.chainlm(e,rp,tp)
fn chainlm(_d: &mut Duel, a: Args) -> bool {
    // return tp==rp
    a.int(2) == a.int(1)
}
// function s.naop(e,tp,eg,ep,ev,re,r,rp)
fn naop(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetTargetRange(0,1)
    effect::set_target_range(d, e1, 0, 1);
    // e1:SetValue(s.aclimit)
    effect::set_value_fn(d, e1, lua_fn(aclimit));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
// function s.aclimit(e,re,tp)
fn aclimit(d: &mut Duel, a: Args) -> bool {
    // return re:GetHandler():IsOnField() or re:IsHasType(EFFECT_TYPE_ACTIVATE)
    let re = a.e(1);
    let h = effect::get_handler(d, re).nil();
    card::is_on_field(d, h) || effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
}
// function s.desinfo(e,tp,eg,ep,ev,re,r,rp)
fn desinfo(d: &mut Duel, a: &EffArgs) {
    // local dg=Duel.GetFieldGroup(tp,0,LOCATION_ONFIELD)
    let dg = duel::get_field_group(d, a.tp, 0, LOCATION_ONFIELD);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,dg,#dg,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg.clone()), dg.len() as u8, 0, 0);
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // local dg=Duel.GetFieldGroup(tp,0,LOCATION_ONFIELD)
    let dg = duel::get_field_group(d, a.tp, 0, LOCATION_ONFIELD);
    // if #dg>0 then
    if !dg.is_empty() {
        // Duel.Destroy(dg,REASON_EFFECT)
        duel::destroy(d, &dg, REASON_EFFECT, None, None);
    }
}
