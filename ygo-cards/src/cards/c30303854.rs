//! Laval Archer
//! ラヴァル・アーチャー
use super::prelude::*;

pub const ID: u32 = 30303854;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LAVAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Gain an extra normal summon for "Laval" monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_TRIGGER_F+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_TRIGGER_F + EFFECT_TYPE_SINGLE);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetOperation(s.sumop)
    effect::set_operation(d, e1, Some(operation(sumop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special summon itself from GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_DESTROY);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.sumop(e,tp,eg,ep,ev,re,r,rp)`: gain an extra normal summon for "Laval" monsters
fn sumop(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetTargetRange(LOCATION_HAND|LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_HAND | LOCATION_MZONE, 0);
    // e1:SetCode(EFFECT_EXTRA_SUMMON_COUNT)
    effect::set_code(d, e1, EFFECT_EXTRA_SUMMON_COUNT);
    // e1:SetTarget(s.estg)
    effect::set_target(d, e1, Some(effect_card_fn(estg)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `s.estg(e,c)`
fn estg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsSetCard(SET_LAVAL)
    card::is_set_card(d, c, &[SET_LAVAL as u16])
}
/// `s.filter(c,ft)`: check for FIRE monsters
fn filter(d: &mut Duel, c: CardId, ft: i32) -> bool {
    // return c:IsFaceup() and c:IsMonster() and c:IsAttribute(ATTRIBUTE_FIRE) and (ft>0 or c:GetSequence()<5)
    card::is_faceup(d, c)
        && card::is_monster(d, c)
        && card::is_attribute(d, c, ATTRIBUTE_FIRE)
        && (ft > 0 || card::get_sequence(d, c) < 5)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`: activation legality
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc,ft) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc, ft);
    }
    // if chk==0 then return ft>-1 and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false)
    //   and Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil,ft) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return ft > -1
            && card::is_can_be_special_summoned(d, h, a.e, 0, a.tp, false, false, None, None, None)
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, ft)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil,ft)
    let g = duel::select_target(d, a.tp, Some(&|d, c| filter(d, c, ft)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.splimit(e,c,sump,sumtype,sumpos,targetp)`: restricted to FIRE monsters
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return not c:IsAttribute(ATTRIBUTE_FIRE)
    let c = a.c(1);
    !card::is_attribute(d, c, ATTRIBUTE_FIRE)
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`: destroy 1 FIRE monster, and if you do, special summon itself from GY
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and Duel.Destroy(tc,REASON_EFFECT)>0 and c:IsRelateToEffect(e) then
    let destroyed = match tc {
        None => false,
        Some(tc) => card::is_relate_to_effect(d, tc, a.e) && duel::destroy(d, tc, REASON_EFFECT, None, None) > 0,
    };
    if destroyed && card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummonStep(c,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon_step(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
        // Banish it if it leaves the field
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3300)
        effect::set_description(d, e1, 3300);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
        effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetReset(RESET_EVENT|RESETS_REDIRECT)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
        // e1:SetValue(LOCATION_REMOVED)
        effect::set_value(d, e1, LOCATION_REMOVED as i64);
        // c:RegisterEffect(e1,true)
        card::register_effect(d, c, e1, true);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
    // Cannot special summon for rest of turn, except FIRE monsters
    // local e2=Effect.CreateEffect(e:GetHandler())
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // e2:SetTarget(s.splimit)
    effect::set_target(d, e2, Some(lua_fn(splimit)));
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
    // aux.RegisterClientHint(e:GetHandler(),nil,tp,1,0,aux.Stringid(id,3),nil)
    aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 3)), 0, None);
}
