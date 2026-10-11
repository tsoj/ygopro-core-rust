//! Noble Knights of the Round Table
use super::prelude::*;

pub const ID: u32 = 55742055;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NOBLE_KNIGHT, SET_NOBLE_ARMS], ..ScriptInfo::DEFAULT };

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
    // During your End Phase: You can activate each of these effects up to once per turn, ...
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetRange(LOCATION_FZONE)
    effect::set_range(d, e1, LOCATION_FZONE);
    // e1:SetCondition(function(e,tp) return Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e1, Some(condition(|d, a| duel::is_turn_player(d, a.tp))));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_NOBLE_KNIGHT) and c:IsAbleToGrave()
    card::is_set_card(d, c, &[SET_NOBLE_KNIGHT as u16]) && card::is_able_to_grave(d, c, None)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_NOBLE_KNIGHT) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_NOBLE_KNIGHT as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_NOBLE_KNIGHT) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_NOBLE_KNIGHT as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return e:GetLabel()==3 and chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.thfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return effect::get_label(d, a.e) == 3
            && card::is_location(d, chkc, LOCATION_GRAVE)
            && card::is_controler(d, chkc, tp)
            && thfilter(d, chkc);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local noble_knights_count=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsSetCard,SET_NOBLE_KNIGHT),tp,LOCATION_GRAVE|LOCATION_ONFIELD,0,nil):GetClassCount(Card.GetCode)
    let kn = duel::get_matching_group(
        d,
        Some(&aux::faceup_filter(|d, x| card::is_set_card(d, x, &[SET_NOBLE_KNIGHT as u16]))),
        tp,
        LOCATION_GRAVE | LOCATION_ONFIELD,
        0,
        NO_EX,
    );
    let cnt = group::get_class_count(d, &kn, &|d, x| card::get_code(d, x)[0] as i64) as i64;
    // local b1=noble_knights_count>=3 and not c:HasFlagEffect(id+1)
    //     and Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,nil)
    let b1 = cnt >= 3
        && card::get_flag_effect(d, c, ID + 1) == 0
        && duel::is_existing_matching_card(d, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    // local b2=noble_knights_count>=6 and not c:HasFlagEffect(id+2)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp)
    let b2 = cnt >= 6
        && card::get_flag_effect(d, c, ID + 2) == 0
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    // local b3=noble_knights_count>=9 and not c:HasFlagEffect(id+3)
    //     and Duel.IsExistingTarget(s.thfilter,tp,LOCATION_GRAVE,0,1,nil)
    let b3 = cnt >= 9
        && card::get_flag_effect(d, c, ID + 3) == 0
        && duel::is_existing_target(d, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    // local b4=noble_knights_count==12 and not c:HasFlagEffect(id+4)
    //     and Duel.IsPlayerCanDraw(tp,1)
    let b4 = cnt == 12 && card::get_flag_effect(d, c, ID + 4) == 0 && duel::is_player_can_draw(d, tp, 1);
    // if chk==0 then return b1 or b2 or b3 or b4 end
    if a.chk == Some(0) {
        return b1 || b2 || b3 || b4;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)}, {b2,aux.Stringid(id,2)}, {b3,aux.Stringid(id,3)}, {b4,aux.Stringid(id,4)})
    let op = duel::select_effect(
        d,
        tp,
        &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2)), (b3, aux::stringid(ID, 3)), (b4, aux::stringid(ID, 4))],
    )
    .nil();
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op as i64]);
    if op == 1 {
        // e:SetCategory(CATEGORY_TOGRAVE)
        effect::set_category(d, a.e, CATEGORY_TOGRAVE);
        // e:SetProperty(0)
        effect::set_property(d, a.e, 0, 0);
        // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
    } else if op == 2 {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_EQUIP)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON + CATEGORY_EQUIP);
        // e:SetProperty(0)
        effect::set_property(d, a.e, 0, 0);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_EQUIP,nil,1,tp,LOCATION_HAND)
        duel::set_possible_operation_info(d, 0, CATEGORY_EQUIP, None, 1, tp, LOCATION_HAND as i32);
    } else if op == 3 {
        // e:SetCategory(CATEGORY_TOHAND)
        effect::set_category(d, a.e, CATEGORY_TOHAND);
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_GRAVE,0,1,1,nil)
        let g = duel::select_target(d, tp, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), 1, tp, 0);
    } else if op == 4 {
        // e:SetCategory(CATEGORY_DRAW)
        effect::set_category(d, a.e, CATEGORY_DRAW);
        // e:SetProperty(0)
        effect::set_property(d, a.e, 0, 0);
        // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    }
    // c:RegisterFlagEffect(id+op,RESETS_STANDARD_PHASE_END,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,op+5))
    card::register_flag_effect(d, c, ID + op as u32, RESETS_STANDARD_PHASE_END, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, op as u32 + 5));
    true
}
/// `s.eqfilter(c,sc,tp)`
fn eqfilter(d: &mut Duel, c: CardId, sc: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_NOBLE_ARMS) and c:IsEquipSpell() and c:CheckEquipTarget(sc) and c:CheckUniqueOnField(tp)
    card::is_set_card(d, c, &[SET_NOBLE_ARMS as u16])
        && card::is_equip_spell(d, c)
        && card::check_equip_target(d, c, sc)
        && card::check_unique_on_field(d, c, tp, None, None)
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    if op == 1 {
        // Hint(SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then Duel.SendtoGrave(g,REASON_EFFECT) end
        if !g.is_empty() {
            duel::send_to_grave(d, g, REASON_EFFECT, None, None);
        }
    } else if op == 2 {
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp):GetFirst()
        let g = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, a.e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &g);
        // if sc and Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)>0 and Duel.GetLocationCount(tp,LOCATION_SZONE)>0
        //     and Duel.IsExistingMatchingCard(s.eqfilter,tp,LOCATION_HAND,0,1,nil,sc,tp)
        //     and Duel.SelectYesNo(tp,aux.Stringid(id,5)) then
        if let Some(sc) = sc {
            if duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0
                && duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
                && duel::is_existing_matching_card(d, Some(&|d, x| eqfilter(d, x, sc, tp)), tp, LOCATION_HAND, 0, 1, NO_EX)
                && duel::select_yes_no(d, tp, aux::stringid(ID, 5))
            {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
                // local ec=Duel.SelectMatchingCard(tp,s.eqfilter,tp,LOCATION_HAND,0,1,1,nil,sc,tp):GetFirst()
                let eg = duel::select_matching_card(d, tp, Some(&|d, x| eqfilter(d, x, sc, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
                if let Some(ec) = group::get_first(d, &eg) {
                    // Duel.BreakEffect()
                    duel::break_effect(d);
                    // Duel.Equip(tp,ec,sc)
                    duel::equip(d, tp, ec, sc, true, false);
                }
            }
        }
    } else if op == 3 {
        // local tc=Duel.GetFirstTarget()
        let tc = duel::first_target(d).nil();
        // if tc:IsRelateToEffect(e) then Duel.SendtoHand(tc,nil,REASON_EFFECT) end
        if card::is_relate_to_effect(d, tc, a.e) {
            duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
        }
    } else if op == 4 {
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, tp, 1, REASON_EFFECT);
    }
}
