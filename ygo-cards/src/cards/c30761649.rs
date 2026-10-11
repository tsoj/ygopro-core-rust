//! Barian Untopia
use super::prelude::*;

pub const ID: u32 = 30761649;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BARIANS, SET_NUMBER, SET_CXYZ, SET_NUMBER_C, SET_RANK_UP_MAGIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // "Number" monsters between "101" and "107", "CXyz" monsters, and "Number C" monsters you control cannot be destroyed by your opponent's card effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    // e1:SetRange(LOCATION_FZONE)
    effect::set_range(d, e1, LOCATION_FZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(s.protectiontg)
    effect::set_target(d, e1, Some(effect_card_fn(protectiontg)));
    // e1:SetValue(aux.indoval)
    effect::set_value_fn(d, e1, aux::indoval());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Your opponent cannot target "Number" monsters between "101" and "107", "CXyz" monsters, and "Number C" monsters you control with card effects
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e2:SetValue(aux.tgoval)
    effect::set_value_fn(d, e2, aux::tgoval());
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Attach 1 monster your opponent controls to 1 Xyz Monster you Special Summoned with a "Rank-Up-Magic" Spell's effect
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(s.attachcon)
    effect::set_condition(d, e3, Some(condition(attachcon)));
    // e3:SetTarget(s.attachtg)
    effect::set_target(d, e3, Some(cost_target(attachtg)));
    // e3:SetOperation(s.attachop)
    effect::set_operation(d, e3, Some(operation(attachop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.protectiontg(e,c)`
fn protectiontg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // local no=c.xyz_number
    let no = crate::card_info(d, c).xyz_number;
    // return ((c:IsSetCard(SET_NUMBER) and no and no>=101 and no<=107) or c:IsSetCard({SET_CXYZ,SET_NUMBER_C})) and c:IsFaceup()
    ((card::is_set_card(d, c, &[SET_NUMBER as u16]) && no.is_some_and(|n| (101..=107).contains(&n)))
        || card::is_set_card(d, c, &[SET_CXYZ as u16, SET_NUMBER_C as u16]))
        && card::is_faceup(d, c)
}
/// `s.attachconfilter(c,tp,re)`
fn attachconfilter(d: &mut Duel, c: CardId, tp: u8, re: Option<EffectId>) -> bool {
    // return c:IsType(TYPE_XYZ) and c:IsFaceup() and c:IsLocation(LOCATION_MZONE) and c:IsSummonPlayer(tp) and re and re:IsSpellEffect()
    //   and re:GetHandler():IsSetCard(SET_RANK_UP_MAGIC)
    if !(card::is_type(d, c, TYPE_XYZ) && card::is_faceup(d, c) && card::is_location(d, c, LOCATION_MZONE) && card::is_summon_player(d, c, tp)) {
        return false;
    }
    let Some(re) = re else { return false };
    effect::is_spell_effect(d, re) && {
        let h = effect::get_handler(d, re).nil();
        card::is_set_card(d, h, &[SET_RANK_UP_MAGIC as u16])
    }
}
/// `s.attachcon(e,tp,eg,ep,ev,re,r,rp)`
fn attachcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.attachconfilter,1,nil,tp,re)
    let eg = a.eg.clone().nil();
    let (tp, re) = (a.tp, a.re);
    group::is_exists(d, &eg, &|d, c| attachconfilter(d, c, tp, re), 1, NO_EX)
}
/// `s.xyztgfilter(c,e,tp)`
fn xyztgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeEffectTarget(e) and Duel.IsExistingTarget(s.attachfilter,tp,0,LOCATION_MZONE,1,c,e,c,tp)
    card::is_can_be_effect_target(d, c, Some(e))
        && duel::is_existing_target(d, Some(&|d, x| attachfilter(d, x, e, c, tp)), tp, 0, LOCATION_MZONE, 1, c.into())
}
/// `s.attachfilter(c,e,xyzc,tp)`
fn attachfilter(d: &mut Duel, c: CardId, e: EffectId, xyzc: CardId, tp: u8) -> bool {
    // return c:IsCanBeEffectTarget(e) and c:IsCanBeXyzMaterial(xyzc,tp,REASON_EFFECT)
    card::is_can_be_effect_target(d, c, Some(e)) && card::is_can_be_xyz_material(d, c, Some(xyzc), Some(tp), Some(REASON_EFFECT))
}
/// `s.attachtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn attachtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local xyzg=eg:Filter(s.attachconfilter,nil,tp,re):Match(s.xyztgfilter,nil,e,tp)
    let eg = a.eg.clone().nil();
    let (tp, re, e) = (a.tp, a.re, a.e);
    let mut xyzg = group::filter(d, &eg, &|d, c| attachconfilter(d, c, tp, re), NO_EX);
    group::match_(d, &mut xyzg, &|d, c| xyztgfilter(d, c, e, tp), NO_EX);
    // if chk==0 then return #xyzg>0 end
    if a.chk == Some(0) {
        return !xyzg.is_empty();
    }
    // local xyzc=nil
    // if #xyzg==1 then
    let xyzc = if xyzg.len() == 1 {
        // xyzc=xyzg:GetFirst()
        group::get_first(d, &xyzg).nil()
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
        // xyzc=xyzg:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &xyzg, tp, 1, 1, false, NO_EX).nil();
        group::get_first(d, &sel).nil()
    };
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATTACH)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATTACH as u64);
    // local g=Duel.SelectMatchingCard(tp,s.attachfilter,tp,0,LOCATION_MZONE,1,1,xyzc,e,xyzc,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, x| attachfilter(d, x, e, xyzc, tp)), tp, 0, LOCATION_MZONE, 1, 1, false, xyzc.into()).nil();
    // Duel.SetTargetCard(g+xyzc)
    let mut targets = g;
    group::add_card(&mut targets, &CardSet::from_card(xyzc));
    duel::set_target_card(d, targets);
    // e:SetLabelObject(xyzc)
    effect::set_label_object(d, e, Value::Card(xyzc));
    true
}
/// `s.attachop(e,tp,eg,ep,ev,re,r,rp)`
fn attachop(d: &mut Duel, a: &EffArgs) {
    // local xyzc=e:GetLabelObject()
    let xyzc = effect::get_label_object(d, a.e).as_card().nil();
    // if not (xyzc:IsRelateToEffect(e) and xyzc:IsFaceup() and not xyzc:IsImmuneToEffect(e)) then return end
    if !(card::is_relate_to_effect(d, xyzc, a.e) && card::is_faceup(d, xyzc) && !card::is_immune_to_effect(d, xyzc, a.e)) {
        return;
    }
    // local attach_tc=(Duel.GetTargetCards(e)-xyzc):GetFirst()
    let mut rest = duel::get_target_cards(d, a.e).nil();
    group::remove_card(&mut rest, &CardSet::from_card(xyzc));
    let attach_tc = group::get_first(d, &rest);
    // if attach_tc and attach_tc:IsControler(1-tp) and not attach_tc:IsImmuneToEffect(e) then
    if let Some(attach_tc) = attach_tc {
        if card::is_controler(d, attach_tc, 1 - a.tp) && !card::is_immune_to_effect(d, attach_tc, a.e) {
            // Duel.Overlay(xyzc,attach_tc,true)
            duel::overlay(d, xyzc, CardSet::from_card(attach_tc), true);
        }
    }
}
