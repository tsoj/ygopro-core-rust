//! The Arrival Cyberse @Ignister
use super::prelude::*;

pub const ID: u32 = 11738489;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[11738489, TOKEN_IGNISTER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:SetUniqueOnField(1,0,id)
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,nil,3,nil,s.lcheck)
    link::add_procedure(d, c, None, 3, None, Some(group_check(lcheck)), None, None);
    // Gain ATK per each material used
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_operation(d, e1, Some(operation(atkop)));
    card::register_effect(d, c, e1, false);
    // Unnafected by card effects
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_IMMUNE_EFFECT);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_value_fn(d, e2, lua_fn(efilter));
    card::register_effect(d, c, e2, false);
    // Destroy 1 monster and Special Summon 1 Token
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_target(d, e3, Some(cost_target(destg)));
    effect::set_operation(d, e3, Some(operation(desop)));
    card::register_effect(d, c, e3, false);
}
/// `s.lcheck(g,lc,sumtype,tp)`
fn lcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return g:CheckDifferentPropertyBinary(Card.GetAttribute,lc,sumtype,tp)
    group::check_different_property_binary(d, g, &|d, x| card::get_attribute_ctx(d, x, Some(lc), sumtype, Some(tp)) as u64)
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsLinkSummoned() then return end
    if !card::is_link_summoned(d, c) {
        return;
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SET_BASE_ATTACK)
    effect::set_code(d, e1, EFFECT_SET_BASE_ATTACK);
    // e1:SetValue(c:GetMaterialCount()*1000)
    effect::set_value(d, e1, card::get_material_count(d, c) as i64 * 1000);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.efilter(e,te)`
fn efilter(d: &mut Duel, a: Args) -> bool {
    // return te:GetOwner()~=e:GetOwner()
    effect::get_owner(d, a.e(1)) != effect::get_owner(d, a.e(0))
}
/// `s.desfilter(c,hc,tp)`
fn desfilter(d: &mut Duel, c: CardId, hc: CardId, tp: u8) -> bool {
    // local zone=hc:GetLinkedZone(tp)&~ZONES_EMZ
    let zone = card::get_linked_zone(d, hc, Some(tp)) & !ZONES_EMZ;
    // return Duel.GetMZoneCount(tp,c,tp,LOCATION_REASON_TOFIELD,zone)>0
    duel::get_m_zone_count(d, tp, c.into(), Some(tp), Some(LOCATION_REASON_TOFIELD), Some(zone)).0 > 0
}
/// `s.token_data`: the Ignister token (`TOKEN_IGNISTER`, `SET_IGNISTER`, `TYPES_TOKEN`, 0, 0, 1, `RACE_CYBERSE`, `ATTRIBUTE_DARK`)
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_IGNISTER as u16]),
        type_: Some(TYPES_TOKEN),
        attack: Some(0),
        defense: Some(0),
        level: Some(1),
        race: Some(RACE_CYBERSE),
        attribute: Some(ATTRIBUTE_DARK),
    }
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc~=e:GetHandler() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && chkc != effect::get_handler(d, a.e).nil();
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingTarget(s.desfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,c,c,tp)
    //     and Duel.IsPlayerCanSpecialSummonMonster(...) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, cc| desfilter(d, cc, c, a.tp)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, c.into())
            && duel::is_player_can_special_summon_monster(d, a.tp, TOKEN_IGNISTER, token_data(), None, None, 0);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,aux.TRUE,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,c)
    let g = duel::select_target(d, a.tp, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, c.into());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // local zone=e:GetHandler():GetLinkedZone(tp)&~ZONES_EMZ
    let h = effect::get_handler(d, a.e).nil();
    let zone = card::get_linked_zone(d, h, Some(a.tp)) & !ZONES_EMZ;
    // if tc:IsRelateToEffect(e) and Duel.Destroy(tc,REASON_EFFECT)>0
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE,tp,LOCATION_REASON_TOFIELD,zone)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(...) then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::destroy(d, tc, REASON_EFFECT, None, None) > 0
        && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, Some(a.tp), Some(LOCATION_REASON_TOFIELD), Some(zone)).0 > 0
        && duel::is_player_can_special_summon_monster(d, a.tp, TOKEN_IGNISTER, token_data(), None, None, 0)
    {
        // local token=Duel.CreateToken(tp,TOKEN_IGNISTER)
        let token = duel::create_token(d, a.tp, TOKEN_IGNISTER).nil();
        // Duel.SpecialSummon(token,0,tp,tp,false,false,POS_FACEUP,zone)
        duel::special_summon(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, Some(zone));
    }
}
