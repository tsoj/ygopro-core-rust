//! Vicious Astraloud
use super::prelude::*;

pub const ID: u32 = 65815684;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_VISAS_STARFROST],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // "Visas Starfrost" + 1 monster with 1500 ATK/2100 DEF
    // Fusion.AddProcMix(c,true,true,CARD_VISAS_STARFROST,s.matfilter)
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![fusion::Mat::Code(CARD_VISAS_STARFROST), fusion::Mat::F(fusion::fus_mat(|d, c, _fc, _st, _tp| matfilter(d, c)))],
    );
    // Fusion.AddContactProc(c,s.contactfil,s.contactop,true)
    fusion::add_contact_proc(
        d,
        c,
        fusion::Contact {
            material_filter: Some(std::sync::Arc::new(contactfil)),
            material_operation: Some(std::sync::Arc::new(contactop)),
            sumcon: Some(None),
            ..Default::default()
        },
    );
    // Destroy 1 other monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Cannot be destroyed by battle
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.matfilter(c)`
fn matfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsAttack(1500) and c:IsDefense(2100)
    card::is_monster(d, c) && card::is_attack(d, c, &[1500]) && card::is_defense(d, c, &[2100])
}
/// `s.contactfil(tp)`
fn contactfil(d: &mut Duel, tp: u8) -> CardSet {
    // local loc=LOCATION_ONFIELD|LOCATION_GRAVE
    let mut loc = LOCATION_ONFIELD | LOCATION_GRAVE;
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION) then loc=LOCATION_ONFIELD end
    if duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_some() {
        loc = LOCATION_ONFIELD;
    }
    // return Duel.GetMatchingGroup(Card.IsAbleToRemoveAsCost,tp,loc,0,nil)
    duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove_as_cost(d, c, None)), tp, loc, 0, NO_EX)
}
/// `s.contactop(g)`
fn contactop(d: &mut Duel, g: &CardSet, _tp: u8, _c: CardId) {
    // Duel.Remove(g,POS_FACEUP,REASON_COST|REASON_MATERIAL)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST | REASON_MATERIAL, None, None);
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc~=c end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && chkc != c;
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,LOCATION_MZONE,LOCATION_MZONE,1,c) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,c)
    let g = duel::select_target(d, a.tp, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, c.into());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) or Duel.Destroy(tc,REASON_EFFECT)==0 then return end
    if !card::is_relate_to_effect(d, tc, a.e) || duel::destroy(d, tc, REASON_EFFECT, None, None) == 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local val=math.max(tc:GetTextAttack(),tc:GetTextDefense())//2
    let val = card::get_text_attack(d, tc).max(card::get_text_defense(d, tc)).div_euclid(2);
    // if val>0 and c:IsRelateToEffect(e) and c:IsFaceup() then
    if val > 0 && card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // c:UpdateAttack(val,RESET_EVENT|RESETS_STANDARD_DISABLE)
        card::update_attack(d, c, val as i64, Some(RESET_EVENT | RESETS_STANDARD_DISABLE), None, None);
    }
}
