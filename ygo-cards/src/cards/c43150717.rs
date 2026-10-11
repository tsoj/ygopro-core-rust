//! Xyz Lay
//! エクシーズ・レイ
use super::prelude::*;

pub const ID: u32 = 43150717;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_XYZ], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e0=aux.AddEquipProcedure(c)
    let e0 = aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // e0:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e0, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // The equipped monster gains 200 ATK for each Xyz Material attached to a monster on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(function(e) return 200*Duel.GetOverlayCount(0,1,1) end)
    effect::set_value_fn(d, e1, lua_fn(atk_value));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Activate the following effect, based on the equipped monster's card type
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(function(e) return e:GetHandler():GetEquipTarget() end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::get_equip_target(d, h).is_some()
    })));
    // e2:SetCost(s.effcost)
    effect::set_cost(d, e2, Some(cost_target(effcost)));
    // e2:SetTarget(s.efftg)
    effect::set_target(d, e2, Some(cost_target(efftg)));
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e) return 200*Duel.GetOverlayCount(0,1,1) end`
fn atk_value(d: &mut Duel, _a: Args) -> i64 {
    200 * duel::get_overlay_count(d, 0, 1, 1, None) as i64
}
/// `s.effcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn effcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local ec=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let ec = card::get_equip_target(d, h).nil();
    // if chk==0 then return not ec:IsType(TYPE_XYZ) or Duel.CheckRemoveOverlayCard(tp,1,0,2,REASON_COST) end
    if a.chk == Some(0) {
        return !card::is_type(d, ec, TYPE_XYZ) || duel::check_remove_overlay_card(d, a.tp, 1, 0, 2, REASON_COST, None);
    }
    // if ec:IsType(TYPE_XYZ) then
    if card::is_type(d, ec, TYPE_XYZ) {
        // Duel.RemoveOverlayCard(tp,1,0,2,2,REASON_COST)
        duel::remove_overlay_card(d, a.tp, 1, 0, 2, 2, REASON_COST, None);
    }
    true
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_XYZ) and c:IsSpellTrap() and c:IsSSetable()
    card::is_set_card(d, c, &[SET_XYZ as u16]) && card::is_spell_trap(d, c) && card::is_s_setable(d, c, false, None)
}
/// `s.spfilter(c,e,tp,lv)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: u32) -> bool {
    // return c:IsLevel(lv) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_level(d, c, &[lv]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ec=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let ec = card::get_equip_target(d, h).nil();
    // local b1=ec:IsType(TYPE_XYZ) and Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_DECK,0,1,nil)
    let b1 = card::is_type(d, ec, TYPE_XYZ) && duel::is_existing_matching_card(d, Some(&setfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    // local b2=not ec:IsType(TYPE_XYZ) and ec:HasLevel() and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp,ec:GetLevel())
    let b2 = !card::is_type(d, ec, TYPE_XYZ)
        && card::has_level(d, ec)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && {
            let lv = card::get_level(d, ec);
            duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp, lv)), tp, LOCATION_HAND, 0, 1, NO_EX)
        };
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // if b1 then
    if b1 {
        // e:SetLabel(1)
        effect::set_label(d, a.e, &[1]);
        // e:SetCategory(CATEGORY_SET)
        effect::set_category(d, a.e, CATEGORY_SET);
        // Duel.Hint(HINT_OPSELECTED,1-tp,aux.Stringid(id,1))
        duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, aux::stringid(ID, 1));
    } else if b2 {
        // e:SetLabel(2)
        effect::set_label(d, a.e, &[2]);
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // Duel.Hint(HINT_OPSELECTED,1-tp,aux.Stringid(id,2))
        duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, aux::stringid(ID, 2));
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // Set 1 "Xyz" Spell/Trap from your Deck
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
        // local g=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&setfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SSet(tp,g)
            duel::s_set(d, tp, g, None, true);
        }
    } else if op == 2 {
        // Special Summon 1 monster with the same Level as the equipped monster from your hand in Defense Position
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return;
        }
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp,c:GetEquipTarget():GetLevel()):GetFirst()
        let lv = card::get_level(d, card::get_equip_target(d, c).nil());
        let g = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, a.e, tp, lv)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &g);
        // if sc and Duel.SpecialSummonStep(sc,0,tp,tp,false,false,POS_FACEUP_DEFENSE) then
        if let Some(sc) = sc {
            if duel::special_summon_step(d, sc, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None) {
                // Negate its effects
                // sc:NegateEffects(c)
                card::negate_effects(d, sc, c, None, false, None);
            }
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
}
