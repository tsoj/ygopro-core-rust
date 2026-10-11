//! Supermagic Sword of Raptinus
//! ラプテノスの超魔剣
use super::prelude::*;

pub const ID: u32 = 53363708;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c)
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // Cannot target
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    // e1:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e1:SetValue(aux.tgoval)
    effect::set_value_fn(d, e1, aux::tgoval());
    // e1:SetCondition(s.indcon)
    effect::set_condition(d, e1, Some(effect_card_fn(indcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Cannot destroy
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // e2:SetCondition(s.ntcon)
    effect::set_condition(d, e2, Some(effect_card_fn(ntcon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Normal Summon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_SUMMON+CATEGORY_POSITION)
    effect::set_category(d, e3, CATEGORY_SUMMON + CATEGORY_POSITION);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_PHASE|PHASE_BATTLE_START)
    effect::set_code(d, e3, EVENT_PHASE | PHASE_BATTLE_START);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetTarget(s.nstg)
    effect::set_target(d, e3, Some(cost_target(nstg)));
    // e3:SetOperation(s.nsop)
    effect::set_operation(d, e3, Some(operation(nsop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.indcon(e,c)`
fn indcon(d: &mut Duel, e: EffectId, _c: CardId) -> bool {
    // return e:GetHandler():GetEquipTarget():IsAttackPos()
    let h = effect::get_handler(d, e).nil();
    let tc = card::get_equip_target(d, h).nil();
    card::is_attack_pos(d, tc)
}
/// `s.ntcon(e,c)`
fn ntcon(d: &mut Duel, e: EffectId, _c: CardId) -> bool {
    // return e:GetHandler():GetEquipTarget():IsDefensePos()
    let h = effect::get_handler(d, e).nil();
    let tc = card::get_equip_target(d, h).nil();
    card::is_defense_pos(d, tc)
}
/// `s.nstg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn nstg(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_equip_target(d, h);
    // if chk==0 then return tc and tc:IsCanChangePosition()
    //     and Duel.IsExistingMatchingCard(Card.IsSummonable,tp,LOCATION_HAND|LOCATION_MZONE,0,1,nil,true,nil) end
    if a.chk == Some(0) {
        return match tc {
            Some(tc) => {
                card::is_can_change_position(d, tc)
                    && duel::is_existing_matching_card(
                        d,
                        Some(&|d, c| card::is_summonable(d, c, true, None, 0, None)),
                        a.tp,
                        LOCATION_HAND | LOCATION_MZONE,
                        0,
                        1,
                        NO_EX,
                    )
            }
            None => false,
        };
    }
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, tc.map(CardSet::from_card), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SUMMON,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SUMMON, None, 1, 0, 0);
    true
}
/// `s.nsop(e,tp,eg,ep,ev,re,r,rp)`
fn nsop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_equip_target(d, h);
    // if tc and Duel.ChangePosition(tc,POS_FACEUP_DEFENSE,POS_FACEDOWN_DEFENSE,POS_FACEUP_ATTACK,POS_FACEUP_ATTACK)~=0 then
    if let Some(tc) = tc {
        if duel::change_position(
            d,
            tc,
            POS_FACEUP_DEFENSE as u8,
            Some(POS_FACEDOWN_DEFENSE as u8),
            Some(POS_FACEUP_ATTACK as u8),
            Some(POS_FACEUP_ATTACK as u8),
            false,
        ) != 0
        {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SUMMON as u64);
            // local sc=Duel.SelectMatchingCard(tp,Card.IsSummonable,tp,LOCATION_HAND|LOCATION_MZONE,0,1,1,nil,true,nil):GetFirst()
            let sg = duel::select_matching_card(
                d,
                a.tp,
                Some(&|d, c| card::is_summonable(d, c, true, None, 0, None)),
                a.tp,
                LOCATION_HAND | LOCATION_MZONE,
                0,
                1,
                1,
                false,
                NO_EX,
            )
            .nil();
            let sc = group::get_first(d, &sg);
            // if sc then
            if let Some(sc) = sc {
                // Duel.Summon(tp,sc,true,nil)
                duel::summon(d, a.tp, sc, true, None, 0, None);
            }
        }
    }
}
