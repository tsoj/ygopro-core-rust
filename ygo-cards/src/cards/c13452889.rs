//! Vector Scare Archfiend
use super::prelude::*;

pub const ID: u32 = 13452889;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must be properly summoned before reviving
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link summon procedure
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_CYBERSE),2)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_CYBERSE, sc, st, Some(tp)))), 2, None, None, None, None);

    // Special summon back a monster this card destroyed by battle
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYING);
    // e1:SetCondition(aux.bdogcon)
    effect::set_condition(d, e1, Some(aux::bdogcon()));
    // e1:SetCost(s.spcost)
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c,tp,g,zone)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8, g: &CardSet, zone: u32) -> bool {
    // return g:IsContains(c) and (Duel.CheckLocation(c:GetControler(),LOCATION_MZONE,c:GetSequence(),true)
    //     or Duel.GetLocationCount(1-tp,LOCATION_MZONE,tp,LOCATION_REASON_TOFIELD,zone)>0)
    group::is_contains(g, c)
        && (duel::check_location(d, card::get_controler(d, c), LOCATION_MZONE, card::get_sequence(d, c))
            || duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, Some(tp), Some(LOCATION_REASON_TOFIELD), Some(zone)).0 > 0)
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local lg=c:GetLinkedGroup()
    let lg = card::get_linked_group(d, c);
    // local zone=c:GetLinkedZone(1-tp)
    let zone = card::get_linked_zone(d, c, Some(1 - tp));
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.cfilter,1,false,nil,nil,tp,lg,zone) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, Some(&|d, x| cfilter(d, x, tp, &lg, zone)), 1, 1, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.cfilter,1,1,false,nil,nil,tp,lg,zone)
    let g = duel::select_release_group_cost(d, tp, Some(&|d, x| cfilter(d, x, tp, &lg, zone)), 1, 1, false, None, NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // local zone1=c:GetLinkedZone(tp)
    let zone1 = card::get_linked_zone(d, c, Some(tp));
    // local zone2=c:GetLinkedZone(1-tp)
    let zone2 = card::get_linked_zone(d, c, Some(1 - tp));
    // if chk==0 then return c:GetLinkedZone()~=0 and (bc:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,tp,zone1,true)
    //     or bc:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,1-tp,zone2)) end
    if a.chk == Some(0) {
        if card::get_linked_zone(d, c, None) == 0 {
            return false;
        }
        let bc = bc.nil();
        return card::is_can_be_special_summoned(d, bc, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(tp), Some(zone1))
            || card::is_can_be_special_summoned(d, bc, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(1 - tp), Some(zone2));
    }
    // Duel.SetTargetCard(bc)
    let bc = bc.nil();
    duel::set_target_card(d, bc);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,bc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(bc)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:GetLinkedZone()~=0 and tc:IsRelateToEffect(e) then
    if card::get_linked_zone(d, c, None) != 0 && card::is_relate_to_effect(d, tc.nil(), e) {
        let tc = tc.nil();
        // local zone1=c:GetLinkedZone(tp)
        let zone1 = card::get_linked_zone(d, c, Some(tp));
        // local zone2=c:GetLinkedZone(1-tp)
        let zone2 = card::get_linked_zone(d, c, Some(1 - tp));
        // if tc:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,tp,zone1)
        //     and (not tc:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,1-tp,zone2) or Duel.SelectYesNo(tp,aux.Stringid(id,1))) then
        if card::is_can_be_special_summoned(d, tc, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(tp), Some(zone1))
            && (!card::is_can_be_special_summoned(d, tc, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(1 - tp), Some(zone2))
                || duel::select_yes_no(d, tp, aux::stringid(ID, 1)))
        {
            // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP,zone1)
            duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, Some(zone1));
        } else {
            // if Duel.SpecialSummon(tc,0,tp,1-tp,false,false,POS_FACEUP,zone2)~=0 and c:IsRelateToBattle() then
            if duel::special_summon(d, tc, 0, tp, 1 - tp, false, false, POS_FACEUP as u8, Some(zone2)) != 0 && card::is_relate_to_battle(d, c) {
                // Can make a second attack
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetDescription(3201)
                effect::set_description(d, e1, 3201);
                // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
                effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetCode(EFFECT_EXTRA_ATTACK)
                effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
                // e1:SetValue(1)
                effect::set_value(d, e1, 1);
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_BATTLE)
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE, 1);
                // c:RegisterEffect(e1)
                card::register_effect(d, c, e1, false);
            }
        }
    }
}
