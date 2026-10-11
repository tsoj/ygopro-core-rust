//! Goblin Bikers Gone Wild
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 55320758;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GOBLIN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If you control no monsters, or all monsters you control are "Goblin" monsters, you can Normal Summon this card without Tributing
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SUMMON_PROC);
    // e1:SetCondition(s.sumcon)
    effect::set_condition(d, e1, Some(lua_fn(sumcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 Level 4 or lower "Goblin" monster from your Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Attach 1 card on the field to a "Goblin" Xyz Monster you control
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e4, aux::stringid(ID, 2));
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e4:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e4, EVENT_TO_GRAVE);
    // e4:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e4, 1, ID, 1, 0);
    // e4:SetTarget(s.attachtg)
    effect::set_target(d, e4, Some(cost_target(attachtg)));
    // e4:SetOperation(s.attachop)
    effect::set_operation(d, e4, Some(operation(attachop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.sumconfilter(c)`
fn sumconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown() or not c:IsSetCard(SET_GOBLIN)
    card::is_facedown(d, c) || !card::is_set_card(d, c, &[SET_GOBLIN as u16])
}
/// `s.sumcon(e,c,minc,zone)`
fn sumcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    let minc = a.int(2);
    let zone = a.int(3) as u32;
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return minc==0 and c:IsLevelAbove(5) and Duel.GetLocationCount(tp,LOCATION_MZONE,tp,LOCATION_REASON_TOFIELD,zone)>0
    //     and (Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)==0 or not Duel.IsExistingMatchingCard(s.sumconfilter,tp,LOCATION_MZONE,0,1,nil))
    minc == 0
        && card::is_level_above(d, c, 5)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, Some(tp), Some(LOCATION_REASON_TOFIELD), Some(zone)).0 > 0
        && (duel::get_field_group_count(d, tp, LOCATION_MZONE, 0) == 0
            || !duel::is_existing_matching_card(d, Some(&sumconfilter), tp, LOCATION_MZONE, 0, 1, NO_EX))
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_GOBLIN) and c:IsLevelBelow(4) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_GOBLIN as u16])
        && card::is_level_below(d, c, 4)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.attachfilter(c,tp)`
fn attachfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return Duel.IsExistingMatchingCard(s.xyzfilter,tp,LOCATION_MZONE,0,1,c,c,tp)
    duel::is_existing_matching_card(d, Some(&|d, x| xyzfilter(d, x, c, tp)), tp, LOCATION_MZONE, 0, 1, c.into())
}
/// `s.xyzfilter(c,mc,tp)`
fn xyzfilter(d: &mut Duel, c: CardId, mc: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_GOBLIN) and c:IsType(TYPE_XYZ) and c:IsFaceup() and mc:IsCanBeXyzMaterial(c,tp,REASON_EFFECT)
    card::is_set_card(d, c, &[SET_GOBLIN as u16])
        && card::is_type(d, c, TYPE_XYZ)
        && card::is_faceup(d, c)
        && card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), Some(REASON_EFFECT))
}
/// `s.attachtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn attachtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() and s.attachfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && attachfilter(d, chkc, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.attachfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| attachfilter(d, c, tp)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATTACH)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATTACH as u64);
    // Duel.SelectTarget(tp,s.attachfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil,tp)
    duel::select_target(d, tp, Some(&|d, c| attachfilter(d, c, tp)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    true
}
/// `s.attachop(e,tp,eg,ep,ev,re,r,rp)`
fn attachop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SELF)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SELF as u64);
        // local xyzc=Duel.SelectMatchingCard(tp,s.xyzfilter,tp,LOCATION_MZONE,0,1,1,tc,tc,tp):GetFirst()
        let g = duel::select_matching_card(d, tp, Some(&|d, x| xyzfilter(d, x, tc, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, tc.into()).nil();
        let xyzc = group::get_first(d, &g);
        // if not xyzc then return end
        let Some(xyzc) = xyzc else {
            return;
        };
        // Duel.HintSelection(xyzc)
        duel::hint_selection(d, xyzc, true);
        // if xyzc:IsImmuneToEffect(e) or tc:IsImmuneToEffect(e) then return end
        if card::is_immune_to_effect(d, xyzc, a.e) || card::is_immune_to_effect(d, tc, a.e) {
            return;
        }
        // Duel.Overlay(xyzc,tc,true)
        duel::overlay(d, xyzc, tc, true);
    }
}
