//! 古の秘儀
use super::prelude::*;

pub const ID: u32 = 30752324;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 Normal Monster from your hand, Deck, or GY in Defense Position (if you Special Summon from the Deck, it must be Level 4 or lower), or if you control a Normal Monster Card, you can apply 1 of these effects instead
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DESTROY+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_DESTROY + CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.acttg)
    effect::set_target(d, e1, Some(cost_target(acttg)));
    // e1:SetOperation(s.actop)
    effect::set_operation(d, e1, Some(operation(actop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // c:IsType(TYPE_NORMAL) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    //     and (c:IsLevelBelow(4) or not c:IsLocation(LOCATION_DECK))
    card::is_type(d, c, TYPE_NORMAL)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
        && (card::is_level_below(d, c, 4) || !card::is_location(d, c, LOCATION_DECK))
}
fn normal_faceup(d: &mut Duel, c: CardId) -> bool {
    // aux.FaceupFilter(Card.IsOriginalType,TYPE_NORMAL)
    card::is_faceup(d, c) && card::is_original_type(d, c, TYPE_NORMAL)
}
fn acttg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // local mmz_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    let mmz_chk = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    // local monster_card_chk=Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsOriginalType,TYPE_NORMAL),tp,LOCATION_ONFIELD,0,1,nil)
    let monster_card_chk = duel::is_existing_matching_card(d, Some(&normal_faceup), tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    // if chk==0 then
    if a.chk == Some(0) {
        // if mmz_chk and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp) then return true end
        if mmz_chk
            && duel::is_existing_matching_card(
                d,
                Some(&|d, c| spfilter(d, c, e, tp)),
                tp,
                LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE,
                0,
                1,
                NO_EX,
            )
        {
            return true;
        }
        // return monster_card_chk and (Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)>0 or Duel.IsPlayerCanDraw(tp,2)
        //     or (mmz_chk and Duel.IsExistingMatchingCard(Card.IsCanBeSpecialSummoned,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,nil,e,0,tp,false,false)))
        return monster_card_chk
            && (duel::get_field_group_count(d, tp, 0, LOCATION_ONFIELD) > 0
                || duel::is_player_can_draw(d, tp, 2)
                || (mmz_chk
                    && duel::is_existing_matching_card(
                        d,
                        Some(&|d, c| card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)),
                        tp,
                        LOCATION_GRAVE,
                        LOCATION_GRAVE,
                        1,
                        NO_EX,
                    )));
    }
    // if not monster_card_chk then
    if !monster_card_chk {
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,PLAYER_EITHER,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, PLAYER_EITHER as u8, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,1,1-tp,LOCATION_ONFIELD)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, 1 - tp, LOCATION_ONFIELD as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DRAW,nil,1,tp,2)
    duel::set_possible_operation_info(d, 0, CATEGORY_DRAW, None, 1, tp, 2);
    true
}
fn actop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local op=nil
    let op: Option<usize>;
    // local mmz_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    let mmz_chk = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    // if Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsOriginalType,TYPE_NORMAL),tp,LOCATION_ONFIELD,0,1,nil) then
    if duel::is_existing_matching_card(d, Some(&normal_faceup), tp, LOCATION_ONFIELD, 0, 1, NO_EX) {
        // local b1=mmz_chk and Duel.IsExistingMatchingCard(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp)
        let b1 = mmz_chk
            && duel::is_existing_matching_card(
                d,
                Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, e, tp))),
                tp,
                LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE,
                0,
                1,
                NO_EX,
            );
        // local b2=Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>0
        let b2 = duel::get_field_group_count(d, tp, 0, LOCATION_MZONE) > 0;
        // local b3=Duel.IsExistingMatchingCard(Card.IsSpellTrap,tp,0,LOCATION_ONFIELD,1,nil)
        let b3 = duel::is_existing_matching_card(d, Some(&card::is_spell_trap), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
        // local b4=Duel.IsPlayerCanDraw(tp,2)
        let b4 = duel::is_player_can_draw(d, tp, 2);
        // local b5=mmz_chk and Duel.IsExistingMatchingCard(aux.NecroValleyFilter(Card.IsCanBeSpecialSummoned),tp,LOCATION_GRAVE,LOCATION_GRAVE,1,nil,e,0,tp,false,false)
        let b5 = mmz_chk
            && duel::is_existing_matching_card(
                d,
                Some(&aux::necro_valley_filter(|d, c| card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None))),
                tp,
                LOCATION_GRAVE,
                LOCATION_GRAVE,
                1,
                NO_EX,
            );
        // op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,1)}, {b2,aux.Stringid(id,2)}, {b3,aux.Stringid(id,3)}, {b4,aux.Stringid(id,4)}, {b5,aux.Stringid(id,5)})
        op = duel::select_effect(
            d,
            tp,
            &[
                (b1, aux::stringid(ID, 1)),
                (b2, aux::stringid(ID, 2)),
                (b3, aux::stringid(ID, 3)),
                (b4, aux::stringid(ID, 4)),
                (b5, aux::stringid(ID, 5)),
            ],
        );
    } else {
        // op=1
        op = Some(1);
    }
    // if op==1 then
    if op == Some(1) {
        // Special Summon 1 Normal Monster from your hand, Deck, or GY in Defense Position (if you Special Summon from the Deck, it must be Level 4 or lower)
        // if not mmz_chk then return end
        if !mmz_chk {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(
            d,
            tp,
            Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, e, tp))),
            tp,
            LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE,
            0,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
            duel::special_summon(d, &g, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
        }
    } else if op == Some(2) {
        // --● Destroy all monsters your opponent controls
        // local g=Duel.GetFieldGroup(tp,0,LOCATION_MZONE)
        let g = duel::get_field_group(d, tp, 0, LOCATION_MZONE);
        // if #g>0 then Duel.Destroy(g,REASON_EFFECT) end
        if !g.is_empty() {
            duel::destroy(d, &g, REASON_EFFECT, None, None);
        }
    } else if op == Some(3) {
        // --● Destroy all Spells/Traps your opponent controls
        // local g=Duel.GetMatchingGroup(Card.IsSpellTrap,tp,0,LOCATION_ONFIELD,nil)
        let g = duel::get_matching_group(d, Some(&card::is_spell_trap), tp, 0, LOCATION_ONFIELD, NO_EX);
        // if #g>0 then Duel.Destroy(g,REASON_EFFECT) end
        if !g.is_empty() {
            duel::destroy(d, &g, REASON_EFFECT, None, None);
        }
    } else if op == Some(4) {
        // --● Draw 2 cards
        // Duel.Draw(tp,2,REASON_EFFECT)
        duel::draw(d, tp, 2, REASON_EFFECT);
    } else if op == Some(5) {
        // --● Special Summon 1 monster from either GY to your field
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(Card.IsCanBeSpecialSummoned),tp,LOCATION_GRAVE,LOCATION_GRAVE,1,1,nil,e,0,tp,false,false)
        let g = duel::select_matching_card(
            d,
            tp,
            Some(&aux::necro_valley_filter(|d, c| card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None))),
            tp,
            LOCATION_GRAVE,
            LOCATION_GRAVE,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, &g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
