//! Magistus Chorozo
use super::prelude::*;

pub const ID: u32 = 66532962;
pub const INFO: ScriptInfo = ScriptInfo { miracle_synchro_fusion: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // c:AddMustFirstBeFusionSummoned()
    card::add_must_first_be_fusion_summoned(d, c);
    // Fusion Materials: 1 Fusion, Synchro, Xyz, or Link Monster + 1 Spellcaster monster
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsType,TYPE_FUSION|TYPE_SYNCHRO|TYPE_XYZ|TYPE_LINK),aux.FilterBoolFunctionEx(Card.IsRace,RACE_SPELLCASTER))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| {
                card::is_type_ctx(d, c, TYPE_FUSION | TYPE_SYNCHRO | TYPE_XYZ | TYPE_LINK, Some(fc), st, Some(tp))
            })),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_race_ctx(d, c, RACE_SPELLCASTER, Some(fc), st, Some(tp)))),
        ],
    );
    // Fusion.AddContactProc(c,function(tp) return Duel.GetMatchingGroup(Card.IsAbleToGraveAsCost,tp,LOCATION_ONFIELD,0,nil) end,function(g) Duel.SendtoGrave(g,REASON_COST|REASON_MATERIAL) end,nil,nil,nil,nil,false)
    fusion::add_contact_proc(
        d,
        c,
        fusion::Contact {
            material_filter: Some(std::sync::Arc::new(contactfil)),
            material_operation: Some(std::sync::Arc::new(contactop)),
            cannot_be_lizard: Some(false),
            ..Default::default()
        },
    );
    // Negate an attack and make this card gain ATK equal to the attacking monster's until the end of the turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.atktg)
    effect::set_target(d, e1, Some(cost_target(atktg)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `Fusion.AddContactProc` material group: `Duel.GetMatchingGroup(Card.IsAbleToGraveAsCost,tp,LOCATION_ONFIELD,0,nil)`
fn contactfil(d: &mut Duel, tp: u8) -> CardSet {
    duel::get_matching_group(d, Some(&|d, c| card::is_able_to_grave_as_cost(d, c)), tp, LOCATION_ONFIELD, 0, NO_EX)
}
/// `function(g) Duel.SendtoGrave(g,REASON_COST|REASON_MATERIAL) end`
fn contactop(d: &mut Duel, g: &CardSet, _tp: u8, _c: CardId) {
    duel::send_to_grave(d, g.clone(), REASON_COST | REASON_MATERIAL, None, None);
}
/// `s.atktg(e,tp,...,chk,chkc)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ac=Duel.GetAttacker()
    let ac = duel::get_attacker(d);
    // if chkc then return chkc==ac end
    if let Some(chkc) = a.chkc() {
        return Some(chkc) == ac;
    }
    // if chk==0 then return ac:IsOnField() and ac:IsCanBeEffectTarget(e) end
    let ac = ac.nil();
    if a.chk == Some(0) {
        return card::is_on_field(d, ac) && card::is_can_be_effect_target(d, ac, Some(a.e));
    }
    // Duel.SetTargetCard(ac)
    duel::set_target_card(d, ac);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,ac,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(ac)), 1, a.tp, 0);
    true
}
/// `s.atkop(e,tp,...)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetAttacker()
    let tc = duel::get_attacker(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() and Duel.NegateAttack() and c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e)
        && card::is_faceup(d, tc)
        && duel::negate_attack(d)
        && card::is_relate_to_effect(d, c, a.e)
        && card::is_faceup(d, c)
    {
        // local atk=tc:GetAttack()
        let atk = card::get_attack(d, tc);
        // if atk<=0 then return end
        if atk <= 0 {
            return;
        }
        // local prev_atk=c:GetAttack()
        let prev_atk = card::get_attack(d, c);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(atk)
        effect::set_value(d, e1, atk as i64);
        // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // Duel.AdjustInstantly(c)
        duel::adjust_instantly(d, Some(c));
        // if prev_atk>=c:GetAttack() then return end
        if prev_atk >= card::get_attack(d, c) {
            return;
        }
        // if tc:IsAbleToHand() and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        if card::is_able_to_hand(d, tc, None) && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoHand(tc,nil,REASON_EFFECT)
            duel::send_to_hand(d, CardSet::from_card(tc), None, REASON_EFFECT, None);
        }
    }
}
