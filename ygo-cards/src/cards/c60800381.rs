//! Junk Warrior
use super::prelude::*;

pub const ID: u32 = 60800381;
pub const INFO: ScriptInfo = ScriptInfo {
    material: &[CARD_JUNK_SYNCHRON],
    listed_names: &[CARD_JUNK_SYNCHRON],
    material_setcode: &[SET_SYNCHRON as u32],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: "Junk Synchron" + 1+ non-Tuner monsters
    // Synchro.AddProcedure(c,s.tunerfilter,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(
        d,
        c,
        Some(mat_filter(|d, x, sc, st, tp| tunerfilter(d, x, sc, st, tp))),
        1,
        1,
        Some(synchro::non_tuner(None)),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // Make this card gain ATK equal to the total ATK of all Level 2 or lower monsters you currently control
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(function(e) return e:GetHandler():IsSynchroSummoned() end)
    effect::set_condition(
        d,
        e1,
        Some(condition(|d, a| card::is_synchro_summoned(d, effect::get_handler(d, a.e).nil()))),
    );
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tunerfilter(c,lc,stype,tp)`
fn tunerfilter(d: &mut Duel, c: CardId, lc: Option<CardId>, stype: u64, tp: u8) -> bool {
    // return c:IsSummonCode(lc,stype,tp,CARD_JUNK_SYNCHRON) or c:IsHasEffect(20932152)
    card::is_summon_code(d, c, lc, stype, tp, &[CARD_JUNK_SYNCHRON])
        || !card::is_has_effect(d, c, 20932152, None).is_empty()
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not (c:IsRelateToEffect(e) and c:IsFaceup()) then return end
    if !(card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c)) {
        return;
    }
    // local atk=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsLevelBelow,2),tp,LOCATION_MZONE,0,nil):GetSum(Card.GetAttack)
    let g = duel::get_matching_group(
        d,
        Some(&|d, x| card::is_faceup(d, x) && card::is_level_below(d, x, 2)),
        a.tp,
        LOCATION_MZONE,
        0,
        NO_EX,
    );
    let atk = group::get_sum(d, &g, &|d, x| card::get_attack(d, x) as i64);
    // if atk>0 then
    if atk > 0 {
        // --It gains ATK equal to the total ATK of all Level 2 or lower monsters you currently control
        // c:UpdateAttack(atk)
        card::update_attack(d, c, atk, None, None, None);
    }
}
