//! Cyber Jormungardr
use super::prelude::*;

pub const ID: u32 = 19715246;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_CYBER_DRAGON, CARD_POLYMERIZATION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableUnsummonable()
    card::enable_unsummonable(d, c);
    // c:AddMustBeSpecialSummonedByCardEffect()
    card::add_must_be_special_summoned_by_card_effect(d, c);
    // Special Summon this card from your hand, then take 1 "Cyber Dragon" from your Deck, and either Special Summon it or equip it to this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e,tp) return Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>0 end)
    effect::set_condition(d, e1, Some(condition(cond1)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add 1 "Polymerization" from your Deck or GY to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(s.thcost)
    effect::set_cost(d, e2, Some(cost_target(thcost)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function(e,tp) return Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>0 end
fn cond1(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_field_group_count(d, a.tp, 0, LOCATION_MZONE) > 0
}
// s.cyberdragonfilter(c,e,tp,mon_ft,st_ft)
fn cyberdragonfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mon_ft: i32, st_ft: i32) -> bool {
    // return c:IsCode(CARD_CYBER_DRAGON) and (s.spfilter(c,e,tp,mon_ft) or s.eqpfilter(c,tp,st_ft))
    card::is_code(d, c, &[CARD_CYBER_DRAGON]) && (spfilter(d, c, e, tp, mon_ft) || eqpfilter(d, c, tp, st_ft))
}
// s.spfilter(c,e,tp,freezone)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, freezone: i32) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false) and freezone>0
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None) && freezone > 0
}
// s.eqpfilter(c,tp,freezone)
fn eqpfilter(d: &mut Duel, c: CardId, tp: u8, freezone: i32) -> bool {
    // return c:CheckUniqueOnField(tp) and not c:IsForbidden() and freezone>0
    card::check_unique_on_field(d, c, tp, None, None) && !card::is_forbidden(d, c) && freezone > 0
}
// s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local mon_ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mon_ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // local st_ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let st_ft = duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0;
    // if chk==0 then return mon_ft>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.IsExistingMatchingCard(s.cyberdragonfilter,tp,LOCATION_DECK,0,1,nil,e,tp,mon_ft-1,st_ft) end
    if a.chk == Some(0) {
        let e = a.e;
        let tp = a.tp;
        return mon_ft > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None)
            && duel::is_existing_matching_card(
                d,
                Some(&|d, x| cyberdragonfilter(d, x, e, tp, mon_ft - 1, st_ft)),
                a.tp,
                LOCATION_DECK,
                0,
                1,
                NO_EX,
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_EQUIP,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_EQUIP, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0 then
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        // local mon_ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
        let mon_ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
        // local st_ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
        let st_ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
        duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
        // local tc=Duel.SelectMatchingCard(tp,s.cyberdragonfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,mon_ft,st_ft):GetFirst()
        let e = a.e;
        let sel = duel::select_matching_card(
            d,
            tp,
            Some(&|d, x| cyberdragonfilter(d, x, e, tp, mon_ft, st_ft)),
            tp,
            LOCATION_DECK,
            0,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        let tc = group::get_first(d, &sel);
        // if tc then
        if let Some(tc) = tc {
            // local b1=s.spfilter(tc,e,tp,mon_ft)
            let b1 = spfilter(d, tc, e, tp, mon_ft);
            // local b2=s.eqpfilter(tc,tp,st_ft)
            let b2 = eqpfilter(d, tc, tp, st_ft);
            // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,3)},{b2,aux.Stringid(id,4)})
            let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 3)), (b2, aux::stringid(ID, 4))]);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // if op==1 then
            if op == Some(1) {
                // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
                duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            // elseif op==2 then
            } else if op == Some(2) {
                // if Duel.Equip(tp,tc,c) then
                if duel::equip(d, tp, tc, c, true, false) {
                    // local e0=Effect.CreateEffect(tc)
                    let e0 = effect::create_effect(d, tc);
                    // e0:SetType(EFFECT_TYPE_SINGLE)
                    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
                    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
                    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
                    // e0:SetCode(EFFECT_EQUIP_LIMIT)
                    effect::set_code(d, e0, EFFECT_EQUIP_LIMIT);
                    // e0:SetValue(function(e,cc) return cc==c end)
                    effect::set_value_fn(d, e0, lua_fn(move |_d, a: Args| a.card(1) == Some(c)));
                    // e0:SetReset(RESET_EVENT|RESETS_STANDARD)
                    effect::set_reset(d, e0, RESET_EVENT | RESETS_STANDARD, 1);
                    // tc:RegisterEffect(e0)
                    card::register_effect(d, tc, e0, false);
                }
            }
        }
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,5))
    effect::set_description(d, e1, aux::stringid(ID, 5));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return not c:IsRace(RACE_MACHINE) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| !card::is_race(d, c, RACE_MACHINE))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
// s.thcostfilter(c)
fn thcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(CARD_CYBER_DRAGON) and c:IsFaceup() and c:IsAbleToHandAsCost()
    card::is_code(d, c, &[CARD_CYBER_DRAGON]) && card::is_faceup(d, c) && card::is_able_to_hand_as_cost(d, c)
}
// s.thcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thcostfilter,tp,LOCATION_ONFIELD,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thcostfilter), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thcostfilter,tp,LOCATION_ONFIELD,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thcostfilter), a.tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoHand(g,nil,REASON_COST)
    duel::send_to_hand(d, &g, None, REASON_COST, None);
    true
}
// s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(CARD_POLYMERIZATION) and c:IsAbleToHand()
    card::is_code(d, c, &[CARD_POLYMERIZATION]) && card::is_able_to_hand(d, c, None)
}
// s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
// s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&aux::necro_valley_filter(|d, x| thfilter(d, x))),
        tp,
        LOCATION_DECK | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, &g);
    }
}
